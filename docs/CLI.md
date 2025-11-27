# ldj CLI 使用說明

工具名稱：`ldj`

全域選項
- `-v, --verbose`：顯示詳細日誌。
- `--workdir <DIR>`：指定工作目錄。
- `--tmpdir <DIR>`：指定暫存目錄（若為 tmpfs，優先使用）。

子命令

1) `pack`

目的：教師將題目資料打包並加密成單一封裝檔。

範例：

```
ldj pack --input-dir ./problem_src --meta meta.json --out prob-001.ldjpkg
```

常用選項
- `--input-dir <DIR>`：包含 `public/`、`private/`、`problem.pdf` 的題目來源目錄。
- `--meta <FILE>`：`meta.json`（題目描述、編譯/執行指令、測資列表）。
- `--out <FILE>`：輸出封裝檔。
- `--pub-pass PROMPT|--pub-pass-file <FILE>|--pub-pass-env <ENV>`：公開密碼來源。
- `--priv-pass PROMPT|--priv-pass-file <FILE>|--priv-pass-env <ENV>`：私有密碼來源。

註：若使用 `PROMPT`，工具會互動式提示輸入密碼（避免列在 shell 歷史中）。

2) `unpack-public`

目的：學生以公開密碼解出題目說明與 `public/` 測資。

範例：

```
ldj unpack-public --package prob-001.ldjpkg --outdir ./public_view
```

常用選項
- `--package <FILE>`：封裝檔。
- `--outdir <DIR>`：公開資料輸出的目的地。
- `--pub-pass PROMPT|--pub-pass-file <FILE>|--pub-pass-env <ENV>`：公開密碼來源。

3) `grade`

目的：在受控批改環境（例如 VM）中，自動化解密私有測資並對學生程式執行測試，產生 JSON 日誌。

範例：

```
ldj grade --package prob-001.ldjpkg --priv-pass-file /secure/path/priv.pass --src student.cpp --workdir ./grade-run --log ./logs/result.json
```

常用選項
- `--package <FILE>`：封裝檔。
- `--priv-pass-file <FILE>` 或 `--priv-pass PROMPT`：私有密碼來源（自動化請使用 `--priv-pass-file`，檔案權限請設為 `0600`）。
- `--src <FILE>`：學生上傳的原始碼檔。
- `--workdir <DIR>`：批改時的工作目錄（預設為 tmpfs 若可用）。
- `--log <FILE>`：輸出 JSON 格式的批改日誌。

安全建議
- 儘量不要在命令列參數中直接放密碼。
- 自動化時，將私有密碼存放於 VM 內受控檔案（`0600`）或透過管理員手動輸入。
- 優先在 tmpfs 中解密與執行，若不可用則在一般 tmp 目錄中並於執行後覆寫刪除。

進階命令
- `ldj export-public`：將公開部分匯出為 zip（包含 `meta.json` 與 `public/`）。
- `ldj verify`：檢查封裝結構與 header 合法性（不解密 private）。

日誌格式
- 日誌為 JSON，包含 `compile`、`results[]` 以及 `summary`。

範例 workflow
1. 教師：`ldj pack --input-dir ./problem_src --meta meta.json --out prob-001.ldjpkg`
2. 考場：將 `prob-001.ldjpkg` 與私有密碼放入受控 VM。
3. 學生：`ldj unpack-public --package prob-001.ldjpkg --outdir ./public_view`（輸入公開密碼）
4. 學生在 VM 中或透過考場提供介面上傳程式，考場以 `ldj grade` 自動化批改並產生日誌。
