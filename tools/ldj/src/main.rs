use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use rand::RngCore;
use ring::pbkdf2;
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::XChaCha20Poly1305;
use chacha20poly1305::XNonce;
use tempfile::tempdir;
use tar::Builder as TarBuilder;
use flate2::write::GzEncoder;
use flate2::Compression;

const MAGIC: &[u8] = b"LDJPKG1\n";

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Pack {
        #[arg(long)]
        input_dir: PathBuf,
        #[arg(long)]
        meta: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    UnpackPublic {
        #[arg(long)]
        package: PathBuf,
        #[arg(long)]
        outdir: PathBuf,
    },
}

#[derive(Serialize, Deserialize)]
struct HeaderBlob {
    meta: serde_json::Value,
    public: BlobInfo,
    private: BlobInfo,
}

#[derive(Serialize, Deserialize)]
struct BlobInfo {
    salt: String,
    nonce: String,
    cipher_len: usize,
}

fn derive_key_from_password(password: &str, salt: &[u8], out_key: &mut [u8]) {
    // Use PBKDF2-HMAC-SHA256 as a simple KDF for prototype (Argon2 can be swapped later)
    let iterations: u32 = 100_000;
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(iterations).unwrap(),
        salt,
        password.as_bytes(),
        out_key,
    );
}

fn tar_gzip_dir(dir: &PathBuf) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let enc = GzEncoder::new(&mut buf, Compression::default());
    let mut tar = TarBuilder::new(enc);
    tar.append_dir_all(".", dir).context("tar append_dir_all")?;
    let enc = tar.into_inner()?;
    // finish gzip
    let mut enc = enc;
    enc.finish()?;
    Ok(buf)
}

fn encrypt_blob(plaintext: &[u8], password: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    // generate salt and nonce
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    let mut key = [0u8; 32];
    derive_key_from_password(password, &salt, &mut key);

    let cipher = XChaCha20Poly1305::new(&key.into());

    let mut nonce_bytes = [0u8; 24];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| anyhow!("encrypt failed: {:?}", e))?;

    Ok((salt.to_vec(), nonce_bytes.to_vec(), ciphertext))
}

fn decrypt_blob(ciphertext: &[u8], password: &str, salt: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
    let mut key = [0u8; 32];
    derive_key_from_password(password, salt, &mut key);
    let cipher = XChaCha20Poly1305::new(&key.into());
    let nonce = XNonce::from_slice(nonce);
    let plain = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("decrypt failed: {:?}", e))?;
    Ok(plain)
}

fn do_pack(input_dir: PathBuf, meta: PathBuf, out: PathBuf) -> Result<()> {
    // Ensure public & private dirs exist under input_dir
    let public_dir = input_dir.join("public");
    let private_dir = input_dir.join("private");

    let public_blob = tar_gzip_dir(&public_dir).context("create public tar.gz")?;
    let private_blob = tar_gzip_dir(&private_dir).context("create private tar.gz")?;

    // read meta
    let mut meta_bytes = Vec::new();
    File::open(&meta)?.read_to_end(&mut meta_bytes)?;
    let meta_json: serde_json::Value = serde_json::from_slice(&meta_bytes)?;

    // ask for passwords
    println!("Enter public password (students will use this):");
    let pub_pass = rpassword::read_password()?;
    println!("Enter private password (for graders):");
    let priv_pass = rpassword::read_password()?;

    let (pub_salt, pub_nonce, pub_cipher) = encrypt_blob(&public_blob, &pub_pass)?;
    let (priv_salt, priv_nonce, priv_cipher) = encrypt_blob(&private_blob, &priv_pass)?;

    let header = HeaderBlob {
        meta: meta_json,
        public: BlobInfo {
            salt: general_purpose::STANDARD.encode(&pub_salt),
            nonce: general_purpose::STANDARD.encode(&pub_nonce),
            cipher_len: pub_cipher.len(),
        },
        private: BlobInfo {
            salt: general_purpose::STANDARD.encode(&priv_salt),
            nonce: general_purpose::STANDARD.encode(&priv_nonce),
            cipher_len: priv_cipher.len(),
        },
    };

    let header_json = serde_json::to_vec(&header)?;

    let mut outf = File::create(&out)?;
    outf.write_all(MAGIC)?;
    let header_len = (header_json.len() as u64).to_le_bytes();
    outf.write_all(&header_len)?;
    outf.write_all(&header_json)?;

    let pub_len = (pub_cipher.len() as u64).to_le_bytes();
    outf.write_all(&pub_len)?;
    outf.write_all(&pub_cipher)?;

    let priv_len = (priv_cipher.len() as u64).to_le_bytes();
    outf.write_all(&priv_len)?;
    outf.write_all(&priv_cipher)?;

    println!("Wrote package to {}", out.display());
    Ok(())
}

fn do_unpack_public(package: PathBuf, outdir: PathBuf) -> Result<()> {
    let mut f = File::open(&package)?;
    let mut magic = [0u8; 8];
    f.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(anyhow!("invalid package magic"));
    }
    let mut header_len_bytes = [0u8; 8];
    f.read_exact(&mut header_len_bytes)?;
    let header_len = u64::from_le_bytes(header_len_bytes) as usize;
    let mut header_json = vec![0u8; header_len];
    f.read_exact(&mut header_json)?;
    let header: HeaderBlob = serde_json::from_slice(&header_json)?;

    // read public cipher len and blob
    let mut pub_len_bytes = [0u8; 8];
    f.read_exact(&mut pub_len_bytes)?;
    let pub_len = u64::from_le_bytes(pub_len_bytes) as usize;
    let mut pub_cipher = vec![0u8; pub_len];
    f.read_exact(&mut pub_cipher)?;

    // skip private blob reading (we can skip or read to move file pointer)
    // but to leave file consistent, read private length and skip bytes
    let mut priv_len_bytes = [0u8; 8];
    f.read_exact(&mut priv_len_bytes)?;
    let priv_len = u64::from_le_bytes(priv_len_bytes) as usize;
    // seek ahead or read and discard
    let mut _priv_blob = vec![0u8; priv_len];
    f.read_exact(&mut _priv_blob)?;

    println!("Enter public password:");
    let pub_pass = rpassword::read_password()?;

    let pub_salt = general_purpose::STANDARD.decode(&header.public.salt)?;
    let pub_nonce = general_purpose::STANDARD.decode(&header.public.nonce)?;

    let plain = decrypt_blob(&pub_cipher, &pub_pass, &pub_salt, &pub_nonce)
        .context("decrypt public blob")?;

    // write plain tar.gz to temp and extract to outdir
    let td = tempdir()?;
    let tmpfile = td.path().join("pub.tar.gz");
    fs::write(&tmpfile, &plain)?;

    // extract
    let tar_gz = File::open(&tmpfile)?;
    let dec = flate2::read::GzDecoder::new(tar_gz);
    let mut ar = tar::Archive::new(dec);
    ar.unpack(&outdir)?;

    // write meta.json
    let meta_path = outdir.join("meta.json");
    let meta_str = serde_json::to_string_pretty(&header.meta)?;
    fs::write(&meta_path, meta_str)?;

    println!("Unpacked public files to {}", outdir.display());
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Pack { input_dir, meta, out } => do_pack(input_dir, meta, out),
        Commands::UnpackPublic { package, outdir } => do_unpack_public(package, outdir),
    }
}
