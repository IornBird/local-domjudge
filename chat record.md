# Message to Copilot

Do NOT read this file, The content is almost SAME to your context. It's only for convience for recording and writing mutiple line prompt.

# Content

---

---

IornBird: 請閱讀`requirements.md`，以軟體開發團隊的專業需求分析者身分，將我當作客戶，與我討論更清晰的軟體需求。您也可以猜測我的需求細節，讓我評估該猜測是否正確。

---

---

我會先建立工作清單並開始找出需求檔案以讀取其內容。

Created 6 todos

我要搜尋工作區中與需求檔名相符的檔案（嘗試 `requirements.md` 或 requirement.md）。

Searched for files matching `**/requirement*.md`, 1 match

Read [](file:///e%3A/move/D/%E7%A0%94%E7%A9%B6%E6%89%80/local%20domjudge/requirement.md)

我已讀取 requirement.md，現在把「讀取檔案」工作標為完成，並開始擷取功能性需求。

Starting: *擷取功能性需求摘要* (2/6)

進度更新：我已讀取並擷取需求檔案內容，下一步是將需求重點化並列出釐清問題與建議。下面以軟體需求分析師身份，針對您當作客戶的需求做整理、假設與釐清問題（中文，請審閱並回覆）：

**摘錄與重點（從 requirement.md）**

- 最終目標：建立一個「本地離線程式批改器」，學生上傳程式，系統以公開與隱藏測資測試並回報是否通過；此系統在考場環境中無網路可用。
- 使用者角色：學生（可匿名/訪客）、教師（管理者）。
- 工作流程：
  - 教師：上傳題目 PDF、編譯腳本、公開/隱藏測資，產生加密封裝檔（可為 zip 或自定義）。
  - 學生：下載題目（解密公開內容需公開密碼）、上傳程式並選題號進行本地批改、查看各測資結果、下載公開測資。
- 測資格式：以二進制儲存；資料夾 `public` / `private`；檔名為編號，副檔名 `.in` / `.out`（例如 `public/1.in`、`private/2.out`）。
- 隱私保護：隱藏測資保持加密，只有在測試輸入當下解密，執行後需清除/確保解密結果不留電磁紀錄。
- 編譯/執行回饋：
  - 顯示編譯錯誤訊息（若編譯失敗）
  - 執行若為 Runtime-error 顯示錯誤訊息；超時僅標示超時
  - 若執行完畢，為避免洩漏隱藏測資，只回報是否正確（不顯示隱藏輸出）
- 平台與工具：
  - Command-line 操作必須支援
  - 支援至少 C++17、Lex（flex）、Yacc（bison）等（具體版本待討論）
  - 僅需支援 Unix-like 作業系統
- 密碼機制與限制：
  - 運行中不提供密碼更改（需重新編譯程式才能改）
  - 有公開密碼與專用（教師）密碼兩種權限
  - 學生需公開密碼才可解出公開題目/測資；教師輸入專用密碼可解密所有內容

**我對需求的合理假設（請確認）**

- 假設 A：系統是一個本地可執行程式（CLI 為主），而非客戶端-伺服器架構；教師/學生在同一裝置上或使用相同離線環境執行操作。
- 假設 B：教師負責產生並分發封裝檔與公開密碼（考試開始時以非電子方式傳達給學生）。
- 假設 C：隱藏測資必須在測試期間短暫解密到磁碟或內存，並在結束時安全刪除；偏好僅在記憶體中處理以減少磁碟殘留風險，但考量到執行環境，可能需在臨時檔案並立即覆寫刪除。
- 假設 D：系統需提供教師產生封裝檔（打包+加密）與學生解封裝/測試兩套功能。
- 假設 E：多位學生同時在同一台機器上使用的情況不常見；系統可為單使用者（序列使用）設計，若需並行則另行設計沙箱/隔離機制。

請確認哪些假設正確，哪些不正確或需修正。

**主要模糊處與需釐清的問題（請逐項回覆）**

1. 帳號模型：是否需要長期的學生/教師帳號管理（帳戶、密碼、角色），或只以「公開密碼 / 專用密碼」為驗證方式（即無持久帳號）？
2. 密碼與金鑰管理：公開密碼與專用密碼是單純密碼導向的解密（對稱密鑰），還是需使用公鑰基礎設施？密碼如何安全分發與儲存？
3. 封裝檔格式：是否接受標準 `zip`（帶壓密密碼或加密）或需自定義容器格式（含 metadata）？若是 zip，是否有版本/相容性要求？
4. 隱私與擦除要求：您期待如何實現「不留電磁紀錄」？（例如，是否接受在磁碟上寫暫存檔並立即覆寫刪除，或必須完全在記憶體中處理？是否需支援 SSD 安全擦除的保證？）
5. 沙箱與安全：學生程式執行需否在完全隔離沙箱（例如 chroot、容器、seccomp、資源限制）中執行？有無惡意程式執行風險需要嚴格隔離？
6. 同步/並發：是否會有多名學生同時在同一台機器使用？是否需並行批改？
7. 日誌/稽核：是否需要保留批改紀錄（日誌），若有，這些日誌能否包含隱藏測資的輸出或僅紀錄結果？
8. UI 需求：僅 CLI 即可，還是需要簡單 GUI（例如 Electron）或網頁介面（即便離線，以本機佈署）？
9. 編譯器/執行環境版本：C++17、lex、yacc 經指定版本，是否需要內建交叉編譯或只使用系統已安裝的編譯器？
10. 可攜性與安裝：希望以單一二進位檔發佈（靜態編譯）還是安裝套件（apt、rpm、tar）？
11. 測資數量上限、超時與資源限制：每題可有多少測資？單次測試允許的最大執行時間、記憶體限制為何？
12. 教師功能細節：教師上傳「編譯腳本」是任何 shell script 嗎？是否需要檢查/沙箱教師腳本以避免在打包階段執行惡意命令？

請就上述各點回覆或指出您已有預設。

**建議的功能性需求（草稿，可接受修改）**

- MUST（必要）
  - 本地 CLI 工具，支援教師建立封裝（打包 + 加密）與學生在本地解封與測試。
  - 封裝包含題目 PDF、`public/` 與 `private/` 目錄（以 `.in` / `.out` 對應）、以及一個 metadata 檔（指定語言、編譯指令、測資 timeout、檔案命名規則、版本號）。
  - 支援至少 C++17、flex、bison；能接受教師指定的編譯腳本（受限於安全檢查）。
  - 隱藏測資在執行前解密，執行完畢後立即清除解密內容（包含暫存檔與記憶體敏感區）。
  - 顯示編譯錯誤、Runtime-error（含錯誤訊息）、timeout（只顯示超時）、以及是否通過（不顯示隱藏輸出）。
  - 支援下載公開測資（學生可下載對應測資的輸入/輸出）。
  - 不使用網路。
- SHOULD（建議）
  - 提供基本帳號/角色機制（至少教師與學生的分別），或以密碼權限控制存取。
  - 以 `zip` 或自定義容器格式支援版本控制與元資料（推薦採用 zip+明確 metadata JSON）。
  - 支援執行沙箱（如透過容器或系統限制）以避免學生程式破壞主機。
  - 日誌功能（不包含隱藏輸出），並可以匯出成 CSV/JSON。
- NICE TO HAVE（可選）
  - GUI 前端（簡易）或本機 Web UI（不啟用網路）以方便非 CLI 使用者。
  - 封裝簽章（數位簽章）以確保題目完整性（若有信任鏈需求）。
  - 自動化產生測資報告與統計。

**建議的非功能性需求（草稿）**

- 安全性：私有測資必須加密（建議使用 AES-GCM 對稱加密，密鑰來源由教師持有）；系統應最小化在磁碟的敏感殘留。
- 可用性：CLI 必須有清晰的子命令（`pack`、`unpack`、`submit`、`run`、`export`），使用說明與範例。
- 可移植性：支援主流 Unix-like 系統（Linux 發行版，macOS 可選）。
- 可維護性：封裝 meta 請採 JSON 格式，並在版本中明確標記。
- 性能：單位測試最多允許 N 秒（可設定），能在可接受時間內完成批改（視測資數量與時限而定）。

**建議的元檔（metadata）草案（示例）**

- `meta.json` 可能包含：
  - `version`：封裝版本
  - `language`：例如 `"cpp17"`
  - `compile_command`：例如 `"g++ -std=c++17 -O2 -pipe -o solution solution.cpp"`
  - `test_timeout_ms`：單個測資的超時（毫秒）
  - `tests`：陣列，測資編號與屬性（是否公開）
  - `pack_timestamp`、`author`、`hash`（選擇性）
    （我可以幫您產生完整 JSON schema）

**接受準則（範例）**

- 教師能用 CLI 打包題目：輸入題目 PDF、`public/`、`private/`、`meta.json`，輸出一個加密封裝檔。
- 學生能以公開密碼解出題目說明與 `public/` 測資，能上傳程式並對所有測資做測試，結果顯示通過/錯誤/超時，並能下載 `public` 測資。
- 隱藏測資在測試期間被安全使用並在測試後無法被學生讀取（驗證：以教師權限可讀，以學生權限不可讀）。
- 系統在無網路環境能完整運作。
- 支援 C++17 的編譯與執行流程能被驗證（舉例：提供一個樣題與測資測試套件做驗收）。

**風險與技術注意事項**

- 在一般作業系統上，確保「絕對不留電磁紀錄」很難達成（特別是 SSD），需要明確界定可接受的風險與做法（例如覆寫、避免 swap、將敏感資料只保存在匿名 tmpfs 中）。
- 執行未信任程式需沙箱化；否則有被濫用風險（破壞檔案、濫用 CPU/記憶體）。
- 教師提供的「編譯腳本」可能含任意命令，必須嚴格限制或審核以免在封裝或批改階段執行危險動作。
- 若採用 zip 加密，不同平台/工具對 zip 密碼加密相容性需注意；建議使用明確加密格式（例如 AES-GCM）與 metadata。

**建議的下一步**

- 請回覆上方「模糊處與釐清問題」中各項的答案（或指出優先回覆的幾項），特別是：帳號模型、密碼/金鑰管理、封裝格式、隱私擦除要求、是否需要沙箱。
- 我可以根據您的回覆立即：
  - 產生「精確的功能需求規格」（包含 meta.json schema、CLI 子命令定義與範例）
  - 或直接產出「技術設計草案」（包含加密方案、暫存處理、沙箱選項與必要系統需求）
  - 或產出「教師打包與學生測試的逐步使用手冊與 CLI 範例」
- 請告訴我您現在最優先的下個輸出：精確需求文件 / 技術設計 / CLI 原型 / 或其他。

如果您願意，我也可以先根據目前假設直接產出：

- `meta.json` 的 JSON schema + 範例
- CLI 命令清單與範例輸入輸出
- 封裝格式（zip vs custom）的比較建議與推薦實作

您想先從哪一項開始？

---

---

Iornbird: 您提供的合理假設皆正確。以下是我對「模糊與釐清問題」的答案

1. 帳號模型：只使用「公開密碼 / 專用密碼」。因為每次設定密碼皆須重新編譯。

2. 密碼與金鑰管理：目前構想的方式，是以文件設定密碼 → 編譯 → 程式內部無法找到解密密碼 → 分發給學生。也就是有可能需使用非對稱加密(或許無法避免留下加密密碼)。

3. 封裝檔格式：請評估標準zip是否能達到第二項要求。若不行，則需要自訂義。

4. 隱私與擦除要求：目前接受在磁碟上寫暫存檔並立即覆寫刪除。

5. 沙箱與安全：目前不需要做隔離。為確保批改公平，已要求在本地虛擬機運行批改部分。

6. 同步/並發：無，所有程式皆在個人電腦運行。

7. 日誌/稽核：需要保留批改紀錄（日誌），但僅紀錄結果。

8. UI 需求：目前僅 CLI 即可。GUI可以列為後續擴充項目

9. 編譯器/執行環境版本：C++17、lex、yacc 經指定版本，是否需要內建交叉編譯或只使用系統已安裝的編譯器？

10. 可攜性與安裝：以單一二進位檔發佈（靜態編譯）。

11. 測資數量上限、超時與資源限制：無特定限制。若必須設限，測資量30對(公開與隱藏)、3秒應已足夠。記憶體限制仍須測試。

12. 教師功能細節：暫時不須限制，但仍提供以下參考
    
    C++：以標準`gcc`、`g++`指令編譯
    
    lex和yacc(學生只能繳交單一`lex`檔案。假設學生上傳`main.l`)：
    
    ```shell
    #!/bin/bash
    
    mkdir -p build
    cd build
    flex ../main.l && gcc lex.yy.c -o main
    ```

請判斷在上述回應下，哪些部分已相對清晰，足以做技術設計。

---

---

Copilot: 我將更新 TODO 清單狀態為「已完成/進行中」，然後整理哪些需求已足以進入技術設計、哪些還需確認與我的建議。

Starting: *提出假設與詳細需求建議* (4/6)

進度更新：TODO 已更新（第1–3 項完成，第4 項進行中）。下面說明在您回覆後哪些部分已足以做技術設計、哪些還需簡短確認，並提出我的具體建議與下一步。

**已足夠清晰，可直接進入技術設計的部分**

- 使用者角色與權限：教師（專用密碼/可解私有測資/管理）、學生（公開密碼/只能看公開測資與題目/上傳程式）。教師在打包時設定密碼並編譯，密碼變更需重新編譯 — 此模式已明確。
- 主要工作流程：教師打包（題目 PDF、public、private、metadata、編譯腳本），產出封裝檔；考場在本地 VM 做批改（private 測資在 VM 中解密並執行）；學生可用公開密碼解開公開內容並在本地（或由 VM）提交程式。
- 封裝內容與檔案命名規則：`public/` / `private/`、檔案以編號命名、`.in` / `.out` 規則已定。
- 隱私擦除接受「寫暫存檔並立即覆寫刪除」：實作上我們可以在暫存檔寫入後用覆寫再刪除，並建議用 tmpfs（RAM）暫存以降低磁碟殘留風險（若環境允許）。
- 沙箱：目前不需要在應用層沙箱，因為您會在受控 VM 中執行批改；因此設計可以假定批改過程在隔離的 VM 環境內進行（系統設計不強制實作容器化）。
- 並發性與部署：每台機器單一使用者（無同時多學生），可用簡單單執行序設計。
- 日誌：需要保留批改結果日誌（僅紀錄結果，不包含隱藏輸出），該需求已明確。
- UI：CLI 足夠（後續可擴 GUI）。
- 版本/工具支援：必須支援 C++17、lex、yacc，並用您提供的 student lex 編譯腳本作為參考（範例已提供）。
- 發佈形式：單一靜態二進位檔（statically linked）為目標。

**仍需您簡短確認／尚未完全決定的項目**

1. 編譯器來源（您在問題 9 留白）：要我假設並設計為「使用目標系統上已安裝的系統編譯器（g++, flex, bison）」？還是要把編譯器/工具鏈一併打包進靜態二進位發佈？  
   - 我的建議：採用系統已安裝的編譯器（較輕量、避免打包龐大 toolchain）。若您希望完全獨立，後續我們再討論 bundling 方案或 container 化。請回覆偏好。
2. 封裝/加密格式：您請我評估標準 zip 是否能達到保密要求。結論與建議在下方（請確認接受或要我另外設計自訂格式）：
   - 評估摘要：
     - 傳統 ZipCrypto（zip 密碼保護）弱、不建議用於保護私有測資。
     - AES-加密的 ZIP（WinZip AES）安全性好，但平台/工具支援可能有相容性問題（在 Unix 環境通常可用 libzip 或 7-zip 支援 AES）。若要最大相容性與可控性，建議不要完全依賴 ZIP 密碼功能。
   - 建議實作（推薦，因為滿足您的需求並易實作）：
     - 封裝流程採「兩段加密 blob」策略：把 `public/` 打包成一個 tar（或 zip）並用「公開密碼」對該 blob 做 AEAD 加密（如 AES-256-GCM）；把 `private/` 打包成另一個 tar 並用「專用密碼」加密；最後把兩個加密 blob 與 `meta.json`（未加密或只含非敏感 metadata）合成一個封裝檔（例如 `.pkg` JSON/二進位容器）。這達到：
       - 教師與學生可分別以不同密碼解密對應 blob。
       - 不需依賴 ZIP 密碼的弱點，使用通用 AEAD（OpenSSL/ libsodium）實作容易且安全。
     - 若您偏好「單一檔案兼容工具」，我們也可以選擇「zip 檔內分別存放 public.zip（加密）/private.zip（加密）/meta.json」，但要確保使用支援 AES-256 的 zip library 來產生/解密。
   - 請回覆：偏好「自訂容器 + AEAD blobs（我的推薦）」還是「zip-with-AES（兼容性考量）」？
3. 密鑰/密碼管理細節：您提到編譯時設定密碼並分發，這裡需確認：
   - 學生端是否要在執行時手動輸入公開密碼來解密公開 blob？（或打包時把公開 blob 放成未加密？）
   - 教師在 VM 上是否會事先在該 VM 上輸入專用密碼或把專用密碼寫入 VM 環境以便自動化批改？（若想自動化，需在 VM 上安全儲存/輸入密碼）
4. temporary file handling：您接受覆寫刪除，但想確認是否願意在可用時使用 `tmpfs`（更安全）或僅用覆寫刪除（較通用）。
5. 資源限制與超時：您提供建議「最多 30 對測資、3 秒超時」。我會把這設為預設但允許在 `meta.json` 中覆寫。確認同意嗎？

**我建議的具體技術選擇（供您快速決策）**

- 封裝格式：推薦「自訂容器」＝ JSON header + two AEAD-encrypted blobs (public/private tar.gz)，副檔名如 `.ldjpkg`（local domjudge pkg）。方便實作且安全性高。若您執意兼容 zip，我可改用 zip（確保使用 AES-256）。
- 加密：AES-256-GCM（或 libsodium secretbox，AEAD）。使用密碼派生鍵（PBKDF2/Argon2）再做 AEAD 加解密。
- Metadata：`meta.json`（版本、lang、compile_command、timeout_ms、tests 列表、author、timestamp）。此檔可放在容器外層未加密或只部分加密（meta 可含哪些測資是 public/private）。
- 暫存：解密後寫到 `TMPDIR`；若環境支援建議先檢查是否掛載 tmpfs，否則覆寫三輪再刪除。
- 編譯器：預設使用系統上安裝的 `g++`, `flex`, `bison`。（如需 bundling 再討論）
- CLI 介面（草案子命令）：`pack`（教師）、`unpack`（教師/學生對公開部分）、`run` 或 `grade`（在 VM 中解密 private blob 並跑測資）、`export-public`、`verify`（檔案完整性）。我可以列詳細參數與範例。
- 日誌：每次打分產出 JSON 日誌（time, user, problem, test_results[ {id, status, time_ms} ]），並將結果保存在 `logs/`。

**下一步 (我可以立刻做的項目，請選一或多項)**

- A. 生成完整技術設計草案（包含：封裝格式細節、meta.json schema、加解密演算法與命令行參數、暫存擦除策略、日誌格式、示例 CLI 指令與流程） —— 建議優先。
- B. 產生 `meta.json` 的 JSON Schema + 範例檔案（可馬上用於打包工具）。
- C. 提供 `pack/unpack` CLI 原型（命令列表與參數，非程式碼）與範例使用流程。
- D. 實作一個最小可運行的 prototype（單檔靜態 Go 或 Rust/C++ 程式）來做「pack/unpack + AES-GCM blobs」，包含 `meta.json` 範例（注意：若要靜態連結加密庫，我會選擇 Go 或 Rust 可較輕易做靜態二進位）。這會是可執行的 PoC，但需您確認使用哪種語言偏好（Go/Rust/C++）。

請先回覆：

1. 對「編譯器來源」的偏好（系統安裝 vs bundling）；  
2. 對「封裝 + 加密」偏好（推薦的自訂 AEAD-blobs vs zip-with-AES）；  
3. 是否接受我先產出完整技術設計草案（選項 A）？  
4. 是否同意把 `meta.json` 中的預設值設為：max_tests=30, timeout_per_test=3000ms（可覆寫）？

收到您確認後，我將把第4、5、6 個 TODO 項目依選擇進行並把設計草案交付。

---

---

1. 編譯器來源：先偏好系統的編譯器。內部連結可列為後續項目。
2. 封裝/加密格式：
   - 接受您的建議。先嘗試AEAD
3. 密鑰/密碼管理細節：您提到編譯時設定密碼並分發，這裡需確認：
   - 學生端要手動輸入公開密碼才可查看題目。
   - 批改需要自動化。因此，還是需要知道如何儲存隱藏測資的密碼。
4. temporary file handling：若可用，請嘗試使用 `tmpfs`
5. 資源限制與超時：可以預設3秒超時且可覆寫。但測資數量由教師(出題者)決定，理應不必預設數量。

另外，雖然我會偏好C++，但若使用Go或Rust較易完成加密等功能，請使用後者。

最後，請實作您列出「下一步」的A選項

---

---

我要更新 TODO 狀態（第4項完成，第5/6項改為進行中），然後交付完整技術設計草案。

Starting: *建議非功能性需求與優先級* (5/6)

進度更新：我已標示第4項為完成，第5/6項為進行中，下面附上完整的技術設計草案（可直接作為開發說明或交付物）。請閱讀並指示是否要我把此設計轉為 README、JSON Schema 檔案、或開始實作 prototype（Rust/Go）來做 pack/unpack + AEAD。

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

**下一步（我可以馬上做的事）**

- 我現在可以幫您把此設計轉成：
  - 1) 完整 `meta.json` JSON Schema 檔案（`meta.schema.json`）並放入 repo；
  - 2) `ldj` CLI 的命令行參數說明文件（`docs/CLI.md`）；
  - 3) 開始實作 MVP prototype（選 Rust 或 Go），先實作 `pack` / `unpack-public` / `grade`（pack/unpack 的加密層次與 `meta.json`），並在 repo 加上 README.md 與 sample problem。
- 請指示您要我先做哪一項（1/2/3）。若選 3，請確認實作語言：`rust` 或 `go`（我推薦 `rust`）。

---

---

我已將您的草案，放入`specific.md`。
請評估您一次可以執行的工作量，以2 → 1 → 3為優先順序，實作部分或全部任務。實作語言為Rust。

---

---

我要在 repository 中新增 CLI 文件、JSON Schema，並新增 Rust prototype 專案的初始實作（pack / unpack-public）。這會建立多個檔案：`docs/CLI.md`、`meta.schema.json`、以及 `tools/ldj` 的 Rust 程式碼與說明。我將一次新增 CLI 文件、JSON Schema、以及 Rust prototype 的初始專案檔案（含 Cargo.toml 與主程式）。這會建立 `docs/CLI.md`, `meta.schema.json`, 與 `tools/ldj` Rust 專案初稿。

Using "Apply Patch"

Using "Apply Patch"

```

```

Using "Apply Patch"

```

```

Using "Apply Patch"

```

```

Using "Apply Patch"

```

```

Using "Apply Patch"

```

```

Using "Apply Patch"

```

```

我要更新 TODO 清單，標示已完成的文件與 prototype 進度。

Added 3 todos

進度更新（已完成）：

- 已新增 `docs/CLI.md`（CLI 文件）
- 已新增 `meta.schema.json`（metadata JSON Schema）
- 已新增 Rust prototype（`tools/ldj`），實作了 `pack` 與 `unpack-public` 的初版實作，並加入範例 `examples/sample_problem/meta.json` 與專案 README.md。

接下來的建議與下一步（我會依您優先順序繼續）：

- 我可以把 prototype 的 KDF 從 PBKDF2 換成 Argon2（符合先前設計建議）；或先保留 PBKDF2 作為可運行的 MVP，再在下一次改進中替換為 Argon2。您偏好哪個路徑？（建議：先以 Argon2 實作，若環境相容性有問題再回退 PBKDF2。）
- 接著我會實作 `grade` 子命令（自動化私有測資解密、在 tmpfs 中展開、編譯學生程式並針對 `meta.json` 的 `tests` 執行測資，輸出 JSON 日誌），並加入安全的暫存/覆寫流程與 `--priv-pass-file` 支援，然後在本地測試編譯（如果您允許我嘗試 build，我會嘗試 cargo build 並回報任何編譯錯誤）。

你想我下一步先：

- A) 將 KDF 換成 Argon2（在 Rust 專案中切換並修正程式碼與 Cargo 依賴）  
- B) 開始實作並測試 `grade` 子命令（包含 tmpfs 使用與日誌）  
- C) 兩者同時（我會先做 Argon2，接著實作 grade；可能需要更多迭代來修正編譯問題）  

請選一項（A / B / C）或提供其他指示。

Made changes.