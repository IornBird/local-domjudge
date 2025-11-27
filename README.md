# local-domjudge
Domjudge when NCU CSIE wi-fi broken

## INFO
由於本人不擅長使用GitHub，有特定更動請在LINE發訊息。

為確保加密功能實作快速，此程式以Rust實作

此程式尚未通過測試，欲加入開發，建議閱讀以下文件
- `docs/CLI.md`: 設定的操作方式
- `specific.md`: 計畫的實作方式
- `requirement.md`: 概略需求

## 進度
增加了用於打包和批改的 CLI 工具的初始實現
- 實作了 `pack` 命令，用於建立包含公共和私有測試資料的加密包檔案。
- 實作了 `unpack-public` 命令，用於從套件檔案中提取公共測試資料和元資料。
- 在 `meta.json` 中新增了元資料模式，用於問題描述和測試配置。
- 引入了使用 PBKDF2 進行金鑰派生和使用 XChaCha20-Poly1305 進行加密的安全密碼處理機制。
- 建立了關於 CLI 用法和命令選項的全面文件。
- 建立了用於評分結果的結構化日誌格式。
- 在 Cargo.toml 中設定了專案依賴項和初始 Rust 配置。
