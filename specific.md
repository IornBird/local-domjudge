**概覽**
- 目標：實作一個離線、本地可執行的 CLI 工具，用以教師建立「封裝檔」並以不同密碼保護公開/隱藏測資；學生輸入公開密碼查看題目/public 測資；考場在受控 VM 中使用專用密碼自動化批改 private 測資並記錄結果。使用 AEAD 加密（推薦 XChaCha20-Poly1305 或 AES-256-GCM + Argon2id 密碼派生）。
- 關鍵設計點：安全的 password->key 派生、AEAD 加密、tmpfs 優先暫存與覆寫刪除策略、metadata (`meta.json`) 驗證、清楚 CLI 子命令、JSON 日誌輸出、系統編譯器依賴（使用系統安裝的 g++/flex/bison）。

**封裝檔格式（建議）**
- 檔名副檔名：`<problem-id>.ldjpkg`（ldjpkg = local domjudge package）
- 容器模型（高階）：單一檔案容器包含：
  - 固定魔術字與版本（ASCII）：`LDJPKG\nv1\n`
  - 一個 JSON header（長度前置）包含 `meta`（非敏感或部分敏感），以及每個 blob 的參考（salt, nonce length, tag length, ciphertext length 等）
  - 一個或多個 AEAD-encrypted blobs（`public_blob`, `private_blob`）以二進位附加（length-prefixed）
- 具體 binary layout（簡潔描述，開發時可依語言 library 實作）：
  - magic(8) | version(4) | header_len(8) | header_json (header_len bytes) | public_blob_len(8) | public_blob(bytes) | private_blob_len(8) | private_blob(bytes)
  - `header_json` 包含：`{"meta":{...}, "public": {"kdf_salt":base64,..., "nonce":base64,..., "cipher_len":n}, "private": {...}}`
- 為了簡化實作與相容，替代選項：把整個 package 做成 `zip`，裡面放 `meta.json`, `public.blob`, `private.blob`（每個 blob 是 AEAD ciphertext）—優點：方便用標準工具檢視；缺點：需確保 zip library 不會改變 ciphertext。

**加密與 KDF（建議）**
- 密碼派生（password -> key）：
  - 使用 Argon2id（推薦參數：time=2, memory=64MiB, parallelism=1；可配置）來派生 32 byte key，並為每個 blob 使用不同 salt（salt 存於 header）。
- AEAD 演算法（推薦順序）：
  1. XChaCha20-Poly1305 (libsodium) — 長 nonce，免擔心 nonce 管理，易於實作且快速，適合多語言支援。
  2. AES-256-GCM — 廣泛支援，但需注意 nonce 長度（96-bit）與安全使用。
- 每個 blob 的資料格式（內部）：
  - kdf_salt (16 bytes)
  - kdf_params (optional json)
  - nonce (24 bytes if XChaCha20; 12 if AES-GCM)
  - ciphertext (包含 tag)
- 對於 metadata integrity：在 header 中可放置 `public_blob_sha256` 與 `private_blob_sha256`（計算在加密前的 plaintext tarball），用於驗證封包完整性（但封包內的 sha256 也會暴露檔案大小/數量資訊；若不要洩露，則僅用 ciphertext tag 作完整性保證）。

**Metadata (`meta.json`)：JSON Schema（草案）**
- 簡要說明：放在 header.meta，描述用於批改的規則、編譯指令、測資列表、預設 timeout 等。
- 範例 schema（重點欄位，完整 schema 可另檔）：
  - `version` (string) — package schema version, e.g., "1.0"
  - `problem_id` (string)
  - `title` (string)
  - `author` (string)
  - `pack_timestamp` (ISO8601)
  - `language` (string) — e.g., `"cpp17"`, `"lex"`, `"yacc"`
  - `compile_command` (string) — e.g., `"g++ -std=c++17 -O2 -pipe -o solution solution.cpp"`
  - `run_command` (string) — e.g., `"./solution"`
  - `tests` (array of objects) — each: `{ "id": "1", "public": true|false, "in": "1.in", "out": "1.out", "timeout_ms": 3000, "memory_kb": null }`
  - `defaults` — e.g., `{ "timeout_ms": 3000 }`
- 範例 `meta.json`：
```json
{
  "version": "1.0",
  "problem_id": "prob-001",
  "title": "Example Problem",
  "author": "Teacher A",
  "pack_timestamp": "2025-11-27T12:00:00Z",
  "language": "cpp17",
  "compile_command": "g++ -std=c++17 -O2 -pipe -o solution solution.cpp",
  "run_command": "./solution",
  "tests": [
    { "id": "1", "public": true,  "in": "public/1.in", "out": "public/1.out", "timeout_ms": 3000 },
    { "id": "2", "public": false, "in": "private/2.in","out": "private/2.out", "timeout_ms": 3000 }
  ],
  "defaults": { "timeout_ms": 3000 }
}
```

**CLI 子命令設計（草案）**
- 工具命名：`ldj`（local domjudge CLI）
- 全域選項：`--verbose`, `--workdir`, `--tmpdir`
- 子命令與示例：
  - `ldj pack --input-dir ./problem_src --meta meta.json --pub-pass PROMPT --priv-pass PROMPT --out prob-001.ldjpkg`
    - `--pub-pass PROMPT` 與 `--priv-pass PROMPT` 表示互動式提示，不在 shell 歷史留痕。也支援 `--pub-pass-env ENVVAR` 或 `--pub-pass-file /path`（建議僅供自動化時使用）。
  - `ldj unpack-public --package prob-001.ldjpkg --pub-pass PROMPT --outdir ./public_view`
    - 學生使用此命令並輸入公開密碼來取得 `meta.json` + `public/` 測資 + PDF 題目。
  - `ldj grade --package prob-001.ldjpkg --priv-pass-file /secure/path/priv.pass --src student.cpp --workdir ./grade-run --log ./logs/result.json`
    - 在 VM / 批改環境自動化執行時，建議使用 `--priv-pass-file` 或 `--priv-pass-env`（使用檔案或環境變數避免命令列記錄）。`grade` 會：
      - 以 `tmpfs` 嘗試建立臨時資料夾，解密 `private_blob` 到該 tmpfs，展開測資，編譯 student code（使用 meta.compile_command，或教師提供的編譯腳本），針對每個 test 用指定 timeout 與 memory 限制執行，生成 JSON log，然後覆寫並刪除解密資料。
  - `ldj export-public --package prob-001.ldjpkg --pub-pass PROMPT --out ./public.zip`
  - `ldj verify --package prob-001.ldjpkg` — 檢查 header/structure格式（不解密 private），可檢查 ciphertext 的整體合法性。

- 密碼輸入方式（建議）：
  - 互動式 prompt（最安全, 避免歷史）
  - 從檔案讀取（`--priv-pass-file /path`，CI/VM 自動化時可事先把密碼放到受控位置）
  - 從 environment variable（`--priv-pass-env LDJ_PRIV_PASS`，較不安全但可用於短期 VM）

**自動化批改中的密碼儲存建議**
- 由於批改需自動化，`private` 密碼必須存在 grader VM 可讀的位置。建議：
  - 在考試前教師將 `private pass` 以受控方式寫入 VM（例如透過主管理者在 VM 建立一個只有 root 或 grade-user 可讀的檔案，權限 `0600`），或教師在考場開始時手動在 VM 以 prompt 輸入一次，之後 VM 內部進行批改。
  - 不建議把密碼以純文字放在共享位置或 shell 歷史中。
  - 若有更高安全性需求，可在 VM 內用簡單的密鑰管理（例如 GPG 加密該檔案並在開始時由教師解鎖），但這會增加流程複雜度。

**暫存與擦除策略**
- 優先使用 tmpfs（建議路徑 `/dev/shm/ldj-XXXXXX` 或 `--tmpdir` 指向 tmpfs）：
  - 實作步驟：
    1. `mkdtemp` 在 `/dev/shm`（若可寫）或 OS tmpdir。
    2. 解密 blob 到該目錄並展開（tar.gz）。
    3. 在批改完成後，對每個已寫的檔案做覆寫（至少一遍零值寫入；若需要更高安全性可覆寫多次），然後 fsync、unlink。
    4. 刪除目錄並嘗試 `shred`/覆寫（若支持）。
- 若 tmpfs 不可用，使用 system tmpdir，但提示使用者檢查 swap 和 SSD 風險，並建議在 VM 或專機上運行（受控），同時覆寫刪除。
- 注意：無法完全保證 SSD 或 OS 內核層面不留下殘留（TRIM, wear-leveling）。在設計文件中須說明 limit。

**批改執行流程（grade）**
1. 建立 tmpdir（prefer tmpfs）
2. 以提供的 `--priv-pass` 派生 key，AEAD 解密 `private_blob` 到 tmpdir，解包 private tests
3. 將 student 源碼複製到 tmpdir（或 teacher 指示的 build dir）
4. 編譯：執行 `compile_command`（可允許教師提供自定義編譯 script），capture stdout/stderr
   - 如果編譯失敗：記錄 compile error 並結束
5. 對每個 test：
   - 啟動沙箱/子進程（注意：您選擇不在應用層做隔離，因為會在 VM 中運行；若未在 VM，建議啟用 OS 限制），設定超時（meta 或 test override），記錄 runtime exit code、stderr
   - 若 runtime error：記錄 `runtime-error` 與 stderr
   - 若 timeout：記錄 `timeout`
   - 若正常結束：比較 stdout 與 `expected output`（binary/bytewise 或 line-based 可選）：
     - 若一致：標記 `accepted`（但若 test 屬 private，學生界面僅顯示 `accepted` / `wrong` 而不顯示輸出）
     - 若不一致：`wrong`
6. 生成 JSON log（見下）
7. 覆寫並刪除 tmpdir

**日誌（log）格式（JSON，範例）**
```json
{
  "problem_id": "prob-001",
  "student_id": null,
  "timestamp": "2025-11-27T12:34:56Z",
  "compile": { "success": true, "stdout": "", "stderr": "" },
  "results": [
    { "test_id": "1", "public": true,  "status": "accepted", "time_ms": 123 },
    { "test_id": "2", "public": false, "status": "wrong",    "time_ms": 15 }
  ],
  "summary": { "passed": 1, "total": 2 },
  "notes": ""
}
```
- 注意：日誌不得包含 private 測資的 plaintext output，僅記錄 `status` 與必要的錯誤訊息（runtime stderr allowed）。

**預設值與可覆寫**
- `timeout_per_test` default = 3000 ms（可由 `meta.json` 或 CLI 選項覆寫）
- `max_tests` 沒有硬性預設（由題目決定），系統會根據 `meta.tests` 列表執行該題測資。
- `tmpfs` 嘗試優先，如果不可用 fallback 至 OS tmpdir。

**安全性與風險清單（需知）**
- SSD 與 OS 可能導致覆寫無法保證物理擦除；需在運行環境（VM）層面控制風險。
- 使用命令列輸入密碼會出現在歷史，應避免；使用 prompt/env/file 來處理。
- 教師提供的編譯腳本若包含惡意命令，會有風險—但您設計中採 VM 執行批改以降低主機風險。
- Argon2 參數需依 VM/環境資源調整，以免過度消耗 CPU/memory。

**接受準則（改寫成可驗收項目）**
- 教師能執行 `ldj pack` 輸出 `prob-xxx.ldjpkg`，內容包含 `meta`、`public_blob`、`private_blob`（encrypted）
- 學生能執行 `ldj unpack-public --pub-pass` 並在 `outdir` 看到題目 PDF、`meta.json` 以及 `public` 測資
- 在 VM 中使用 `ldj grade --priv-pass-file ... --src student.cpp` 能：
  - 解密 private 測資、編譯與執行學生程式
  - 對每個測資產生 `accepted/wrong/runtime-error/timeout`
  - 產生 JSON log，且 log 不包含 private 明文輸出
  - 解密檔案在 grade 完成後被覆寫刪除
- 工具應用於 Unix-like 系統（Linux），並使用系統已安裝 `g++`, `flex`, `bison` 來完成編譯（若沒有，tool 要友善報錯）

**開發/交付建議與優先順序**
- MVP（里程碑 1）：實作 `pack` / `unpack-public` / `grade` 的 CLI，使用 libsodium（或 Rust 的 `sodiumoxide` / Go 的 `golang.org/x/crypto` + `filippo.io/age`）完成 key derivation + AEAD，加上 `meta.json` schema。完成基本流程與日誌。
- 里程碑 2：加入 tmpfs 實作、覆寫刪除策略、更多 KDF 參數設定、CLI 安全選項（pass-file、env）
- 里程碑 3：測試套件、示範題、教學文件、將二進位置為靜態編譯發佈（Rust/Go 都可以做靜態）
- 可選（後續）：加入數位簽章、GUI、支援更多語言沙箱化（container）

**實作語言建議**
- 建議使用 Rust 或 Go：
  - Rust：生態成熟（`ring`, `sodiumoxide`, `argon2` crates），可做小而安全的靜態二進位；錯誤處理嚴謹，適合安全工具。
  - Go：標準庫齊全，執行檔單一檔案、交叉編譯方便，且有 libsodium wrapper 可用；但靜態鏈接有時需注意 cgo。
- 您先前允許以 Go 或 Rust 做實作；對以 AEAD（XChaCha20）與 Argon2 實作，Rust 生態會比較成熟。