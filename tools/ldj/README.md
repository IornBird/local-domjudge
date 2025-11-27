# ldj prototype (Rust)

This is a minimal prototype for the `ldj` CLI (Local DOMjudge packer/grader).

Implemented features in prototype:
- `pack` : package `public/` and `private/` directories into a single `.ldjpkg` file; encrypt blobs with XChaCha20-Poly1305; KDF via PBKDF2 (prototype).
- `unpack-public` : prompt for public password and extract `public/` files and `meta.json` to `outdir`.

Notes:
- This is an MVP prototype. For production, replace PBKDF2 with Argon2 and add `grade` implementation, tmpfs handling, secure overwrites, and stronger KDF parameters.
- Build with Rust 1.70+ (recommended).

Build
```
cd tools/ldj
cargo build --release
```

Basic usage
```
./target/release/ldj pack --input-dir ../examples/sample_problem --meta ../examples/sample_problem/meta.json --out prob-001.ldjpkg
./target/release/ldj unpack-public --package prob-001.ldjpkg --outdir ./public_view
```
