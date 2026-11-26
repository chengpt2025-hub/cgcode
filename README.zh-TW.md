<div align="center">

<img src="https://raw.githubusercontent.com/chengpt2025-hub/cgcode/v2/desktop/overlay/apps/desktop/resources/icon.png" width="120" alt="cgcode logo" />

# cgcode

### 用你自己的大模型，打造中文世界的 AI 程式設計助手與全能桌面助手

[简体中文](README.md) | **繁體中文** | [English](README.en.md) | [日本語](README.ja.md) | [한국어](README.ko.md)

[下載終端版](https://github.com/chengpt2025-hub/cgcode/releases) · [核心亮點](#核心亮點) · [為什麼選擇-cgcode](#為什麼選擇-cgcode) · [路線圖](#路線圖) · [常見問題](#常見問題)

</div>

---

## 一句話介紹

cgcode 是一個**面向中文使用者、可接入任意大模型的 AI 助手**。它把世界一流的開源 AI Agent 底座（OpenAI Codex、DeepSeek Harness）深度改造成**全中文、零門檻、不繫結賬號、不上報資料**的產品，同時提供程式設計師用的**終端版**和普通人用的**桌面客戶端**。

| | 終端版（第一版） | 桌面客戶端（第二版） |
|---|---|---|
| 適合誰 | 程式設計師、開發者 | 所有人：學生、上班族、創作者、團隊 |
| 能做什麼 | 讀懂專案、寫程式碼、改檔案、跑命令、審查程式碼、管理 Git | 聊天、聯網搜尋、寫文件、做表格和 PPT、生成圖片、程式設計、多智慧體協作、定時任務 |
| 使用方式 | 在終端裡執行 | 雙擊安裝，有完整視窗介面 |
| 程式碼分支 | `main` | `v2` |
| 當前狀態 | 驗收版已釋出 | 驗收版已可下載 |

---

## 核心亮點

### 1. 模型自由：想用哪個就用哪個
- 預設接入商湯**日日新 SenseNova**，開箱即用。
- 支援 DeepSeek、通義千問、Kimi 以及任何**相容 OpenAI 格式**的介面和中轉站。
- 終端版**獨家內建 Chat Completions 適配層**：上游 Codex 已移除 Chat 介面、只支援 Responses 介面，cgcode 讓只支援 Chat 介面的國產模型也能完整驅動 Agent。
- 桌面版可在介面裡**隨時切換模型**。

### 2. 真正的全中文
- 介面、斜槓命令說明、歡迎頁、狀態頁、底部提示、預設回覆，全部是簡體中文。
- 預裝技能用中文編寫，按中文說法觸發（例如“幫我審查程式碼”“提交前檢查”）。

### 3. 零門檻上手
- 第一次啟動有**中文設定嚮導**：填介面地址、Key、模型名，**自動測試連通**，不用改任何配置檔案。
- 桌面版歡迎頁只要填一個 API Key 就能開始用。
- 不需要 ChatGPT、Claude 等海外賬號。

### 4. 隱私優先
- API Key 只儲存在你自己的電腦上。
- 對話只發送到你自己配置的模型服務。
- **預設關閉**使用資料上報、反饋上傳、會話遙測和官方更新檢查。

### 5. 穩定耐用
- 遇到模型服務**限流（429）自動快速重試**。
- 可配置**備用介面**，主介面失敗時自動切換。

### 6. 開箱即用的技能生態
- 終端版預裝 **6 個**中文開發技能；桌面版預裝 **38 個**技能（19 個自研中文技能 + 19 個開源市場精選熱門技能）。
- 桌面版內建**中文技能市場**：一句話搜尋 skills.sh、ClawHub、國內 SkillHub，結果中文展示，**安全檢查後一鍵安裝**。
- 每個預裝技能都核對過開源許可證並做過安全審查，**拒絕“破甲 / 越獄”類技能**，保護你的賬號和 Key 不被封。

### 7. 好看
- **iOS 風格設計**：圓角、留白、iOS 系統配色；終端版 7 套主題，用 `/theme` 即時預覽切換。
- 官方 **10 秒開機動畫**：地球 → 蟲洞 → 機械隧道 → 光爆 → CG CODE / SYSTEM ONLINE，配激昂音樂。

### 8. 全能 + 多智慧體（桌面版）
- 一個應用搞定聊天、搜尋、Word / Excel / PPT、圖片、程式設計。
- 參考 Grok 的**多智慧體協作**：複雜任務自動拆分給多個智慧體並行完成。
- **定時任務**、**可插拔外掛**，按需擴充套件。

---

## 為什麼選擇 cgcode

| 對比項 | cgcode | 官方 Codex CLI | ChatGPT / Claude 等海外客戶端 |
|---|---|---|---|
| 可用模型 | 任意相容 OpenAI 格式的模型，預設國產日日新 | 主要面向 OpenAI 模型，僅支援 Responses 介面 | 只能用自家模型 |
| 只支援 Chat 介面的國產模型 | 支援（內建適配層） | 不支援 | 不支援 |
| 介面語言 | 全中文 | 英文 | 多語言 |
| 賬號要求 | 不需要，填自己的 Key 即可 | 需要 OpenAI 賬號或 Key | 需要海外賬號 |
| 首次設定 | 中文嚮導，自動測連通 | 手動配置 | 登入賬號 |
| 使用資料上報 | 預設全部關閉 | 視官方設定而定 | 視官方政策而定 |
| 預裝中文技能 | 終端版 6 個 / 桌面版 38 個 + 中文技能市場 | 無 | 無 |
| 普通人能用的桌面版 | 有 | 無 | 有 |

---

## 快速開始

### 終端版

1. 開啟 [Releases 頁面](https://github.com/chengpt2025-hub/cgcode/releases)，下載最新的 `cgcode-windows-x64.zip`（約 186 MB），解壓到任意資料夾，例如 `D:\cgcode`。
2. 開啟“終端”或“PowerShell”，進入你的專案資料夾，執行 `D:\cgcode\cgcode.exe`。
   - 程式暫未做程式碼簽名，若提示“Windows 已保護你的電腦”，點選 **更多資訊 → 仍要執行**。
3. 觀看開機動畫（按任意鍵跳過），按中文嚮導填入：
   - ① 介面地址，例如 `https://token.sensenova.cn/v1`
   - ② API Key
   - ③ 模型名，例如 `sensenova-6.8-flash`
4. 連通測試通過後直接開始對話。

### 桌面客戶端（驗收階段）

1. 登入 GitHub，開啟 [Actions 頁面](https://github.com/chengpt2025-hub/cgcode/actions)，選擇 `v2` 分支最新一次成功的構建。
2. 在頁面底部 **Artifacts** 下載 `cgcode-windows-x64` 並解壓。
3. 雙擊 `cgcode-<版本>-win-x64-unsigned.exe` 安裝；出現 SmartScreen 提示時點 **更多資訊 → 仍要執行**。
4. 首次啟動播放開機影片，然後在歡迎頁填入日日新 API Key 即可使用。

> 正式版將提供帶數字簽名的安裝包，並支援自動更新。

---

## 終端版詳解

### 主題（7 套，`/theme` 切換）

| 主題 | 說明 |
|---|---|
| iOS 深色 | 預設 |
| iOS 淺色 | 適合淺色終端背景 |
| 午夜藍 | 深藍色調 |
| 薄荷綠 | 清新綠色調 |
| 日落橙 | 暖橙色調 |
| 石墨灰 | 低飽和灰色調 |
| 高對比 | 視力輔助、投屏演示 |

每套主題同時統一介面配色與程式碼高亮配色。

### 預裝技能（6 個）

| 技能 | 作用 | 怎麼觸發 |
|---|---|---|
| cg-review | 中文程式碼審查，按嚴重程度列出問題和修改建議 | “審查”“幫我看看程式碼” |
| cg-ship | 釋出前檢查：格式化、編譯、測試、查敏感資訊，生成中文提交說明 | “釋出”“提交前檢查” |
| cg-focus | 焦點板：在 `.cgcode/FOCUS.md` 記錄目標與進度，長任務不跑偏 | “我們做到哪了” |
| cg-memory | 專案記憶：把約定和踩過的坑記到 `.cgcode/MEMORY.md` | “記住”“以後都這樣” |
| cg-git | Git 助手：中文解釋狀態，處理分支、衝突、回退，危險操作先確認 | 提到 git、分支、衝突 |
| cg-context | 上下文管理：總結進度，建議 `/compact` 或 `/new` | “太長了”“總結一下” |

### 配置檔案

位置：`%USERPROFILE%\.cgcode\cgcode.toml`（嚮導自動生成，一般不用手改）。

```toml
[api]
base_url = "https://token.sensenova.cn/v1"
api_key  = "你的 Key"
model    = "sensenova-6.8-flash"

# 可選：備用介面，主介面失敗時自動切換
# [[fallback]]
# base_url = "https://api.deepseek.com/v1"
# api_key  = "..."
# model    = "deepseek-chat"

[intro]
enabled = true   # 改成 false 關閉開機動畫
sound   = true   # 改成 false 關閉開機聲音
```

---

## 桌面客戶端詳解

### 內建能力
- **多模型切換**：預設日日新 `sensenova-6.8-flash`，預置通義千問（qwen-plus、qwen-max）、Kimi（moonshot-v1-128k），可新增任意相容 OpenAI 格式的模型。
- **全能**：聊天、聯網搜尋、Word / Excel / PPT、圖片生成、程式設計。
- **多智慧體協作**、**定時任務**、**外掛管理**。
- 原畫質開機影片（1248×704），點選或按任意鍵跳過。
- **v2.1 新增**：
  - 預設開啟**本地語音輸入**(SenseVoice，CPU 即可執行，首次使用需下載模型)、**歷史對話搜尋**、**對話自動標題**、**外掛市場**。
  - **電腦操控**：讓助手操作電腦上的其他軟體(基於 agent-desktop)，危險操作每次先確認。
  - **全網閱讀**：讀 B站、YouTube 字幕、V2EX、雪球、GitHub、RSS 和任意網頁；小紅書等需登入的渠道預設關閉，開啟前提示用小號防封號(基於 Agent-Reach)。
  - **長期記憶**：你明確讓它記住的偏好儲存在本機，換對話也記得。
  - **安心模式**：發訊息、付款、刪除等操作先徵得同意；不碰密碼和驗證碼；網頁裡的文字只當資料、不當指令；做完彙報做了什麼。
- **v2.2 新增**：
  - **本地離線模型**：裝了 Ollama 後模型列表出現“本地離線模型”，不聯網、不需要 API Key，預置通義千問 3.5 4B、DeepSeek R1 8B/14B。
  - **配音與朗讀**：文字轉語音，支援多種方言和外語，可克隆聲音(須本人或已獲授權，檔名帶“AI合成”)，基於 VoxCPM2。
  - **創作者工作室**：選題、劇本、分鏡、角色配音、字幕一條龍。
  - **對話統計條**：顯示輪次、模型與工具耗時、首字等待時間。
- **正式版 1.0.0 新增**：
  - **自動更新**：檢查 cgcode 自己的更新源，不會被底座官方版本覆蓋。
  - **iOS 風格主題**：iOS 藍主按鈕、更大圓角，淺色/深色都適配。
- **v2.3 新增**：
  - **專業軟體操作**：通過 CLI-Anything 操作 GIMP、Blender、LibreOffice 等專業軟體，批次修圖、渲染、轉文件，不覆蓋原檔案。
  - **增強記憶(可選安裝)**：claude-mem 自動記錄做過的事並在以後自動回憶，資料只存本機。
  - 手機遙控需要開放區域網訪問，安全風險較大，移到正式版之後再做。

### 預裝技能（38 個）

**cgcode 自研中文技能（19 個）**

| 技能 | 作用 |
|---|---|
| cg-writing | 中文寫作與潤色 |
| cg-meeting-notes | 會議紀要整理 |
| cg-data-analysis | 資料分析 |
| cg-ppt | 製作 PPT |
| cg-research | 資料調研 |
| cg-code-review | 程式碼審查 |
| cg-team | 多智慧體分工協作 |
| cg-daily-plan | 每日計劃 |
| cg-skill-market | 中文技能市場：搜尋 skills.sh、ClawHub、SkillHub，安全檢查後一鍵安裝 |
| cg-web-scraper | 網頁資料抓取，整理成 Excel |
| cg-multi-search | 多引擎搜尋（百度、必應、搜狗等）彙總去重 |
| cg-web-reach | 全網閱讀：B站、YouTube 字幕、V2EX、雪球、GitHub、RSS、網頁等 |
| cg-desktop-control | 電腦操控：操作 Windows 上的其他軟體，危險操作先確認 |
| cg-long-memory | 長期記憶：把你要求記住的偏好儲存在本機 |
| cg-offline-mode | 離線模式：安裝 Ollama 並按視訊記憶體推薦本地模型 |
| cg-voice-studio | 配音與朗讀：文字轉語音、方言、聲音克隆(須授權) |
| cg-creator-studio | 創作者工作室：劇本→分鏡→配音→字幕 |
| cg-pro-software | 專業軟體操作：GIMP、Blender、LibreOffice 等(CLI-Anything) |
| cg-memory-plus | 增強記憶：可選安裝 claude-mem，資料只存本機 |

**開源市場精選技能（19 個）**

| 技能 | 作用 | 來源 / 許可證 |
|---|---|---|
| find-skills | 搜尋並安裝開源技能 | vercel-labs/skills · MIT |
| agent-browser | 瀏覽器自動化：開啟網頁、點選、填表、截圖 | vercel-labs/agent-browser · Apache-2.0 |
| frontend-design | 有設計感的網頁與介面 | anthropics/skills · Apache-2.0 |
| skill-creator | 建立、測試、改進自己的技能 | anthropics/skills · Apache-2.0 |
| mcp-builder | 開發 MCP 工具服務 | anthropics/skills · Apache-2.0 |
| webapp-testing | 用 Playwright 測試網頁 | anthropics/skills · Apache-2.0 |
| canvas-design | 海報與視覺設計 | anthropics/skills · Apache-2.0 |
| theme-factory | 文件、幻燈片、網頁配色主題 | anthropics/skills · Apache-2.0 |
| web-artifacts-builder | 多元件複雜網頁 | anthropics/skills · Apache-2.0 |
| algorithmic-art | p5.js 生成藝術 | anthropics/skills · Apache-2.0 |
| pdf | PDF 提取、合併拆分、填表、生成 | anthropics/skills · Apache-2.0 |
| internal-comms | 週報、通知等內部文稿 | anthropics/skills · Apache-2.0 |
| humanizer | 去 AI 味：讓文字更像真人寫的 | blader/humanizer · MIT |
| security-audit | 分階段程式碼安全審計 | cloudflare/security-audit-skill · MIT |
| brainstorming | 動手前先把需求問清楚、列出方案 | obra/superpowers · MIT |
| writing-plans | 把任務拆成可執行、可驗證的小步驟 | obra/superpowers · MIT |
| systematic-debugging | 系統化排錯，先找根因 | obra/superpowers · MIT |
| test-driven-development | 先寫測試再寫程式碼 | obra/superpowers · MIT |
| verification-before-completion | 聲稱完成前必須實際驗證 | obra/superpowers · MIT |

### 技能安全原則
- 每個預裝技能都核對過開源許可證，並檢查過附帶指令碼（無竊取 Key、無可疑聯網、無修改系統配置）。
- 開源技能在構建時按**固定提交版本**下載，防止上游被篡改。
- **不預裝、並在技能市場中過濾**繞過模型安全限制的“破甲 / 越獄”類技能：這類技能違反模型服務條款，可能導致 Key 或賬號被封，公開分發也有法律風險。

---

## 技術架構

### 終端版

```
使用者終端
  └─ cgcode.exe（Rust）
       ├─ 中文嚮導 / 開機動畫 / iOS 主題 / 中文介面
       ├─ Codex 核心（讀寫檔案、執行命令、審批機制）── Responses 介面
       └─ cgcode-chat-bridge（本地適配層，127.0.0.1）
              └─ Responses ⇄ Chat Completions 轉換，限流重試，備用介面切換
                    └─ 你的模型 API（日日新 / DeepSeek / 千問 …）
```

| 檔案 | 作用 |
|---|---|
| `codex-rs/cgcode-chat-bridge/` | Chat Completions 適配層（含單元測試） |
| `codex-rs/cli/src/cgcode.rs` | 配置、首次嚮導、預裝技能、啟動本地適配層 |
| `codex-rs/cli/src/cgcode_intro.rs` | 終端開機動畫與聲音 |
| `codex-rs/tui/src/cgcode_theme.rs` | 7 套 iOS 風格主題 |
| `.github/workflows/cgcode-build.yml` | Windows 自動構建與釋出 |

### 桌面客戶端

不直接改上游程式碼，而是構建時“套用定製”，方便持續跟進上游更新：

```
desktop/
├─ DSH_COMMIT              鎖定的上游版本
├─ apply.sh                套用品牌、預設中文、模型配置等定製
├─ patch-bundle.mjs        預設模型、多模型接入、關閉遙測
├─ patch-desktop.mjs       歡迎頁 Key、開機影片、預裝技能安裝
├─ fetch-skills.mjs        按固定版本拉取開源技能並補中文說明
├─ upstream-skills.json    開源技能清單（來源、版本、中文說明）
└─ overlay/                圖示、開機影片、自研中文技能等資源
ci/build-windows.sh        Windows 構建指令碼（Node 24 + pnpm）
```

### 構建與釋出
- 每次推送程式碼，GitHub Actions 自動在 Windows 上編譯。
- 終端版：約 1.5–2 小時，成功後自動釋出到 Releases。
- 桌面版：約 15–25 分鐘，安裝包作為 Artifact 上傳。
- 每次構建結束自動開一個“構建成功 / 構建失敗”的 Issue 通知結果。

---

## 路線圖

### 終端版
- [x] Chat Completions 適配層（相容只支援 Chat 介面的模型）
- [x] 接入日日新，限流自動重試，備用介面切換
- [x] 中文首次啟動向導
- [x] 品牌獨立（cgcode、`~/.cgcode`），移除 ChatGPT 登入與遙測
- [x] 全中文介面
- [x] 7 套 iOS 風格主題 + `/theme`
- [x] 官方開機影片（終端字元畫版）+ 開機音樂
- [x] 6 個預裝中文技能
- [x] Windows 自動構建與釋出
- [ ] 真機驗收
- [ ] 根據驗收反饋修復，釋出穩定版 v1.0
- [ ] 執行日誌功能

### 桌面客戶端
- [x] 品牌改為 cgcode，預設中文介面
- [x] 多模型：日日新（預設）、通義千問、Kimi
- [x] 歡迎頁填 Key，零門檻上手
- [x] 關閉會話遙測和產品資料上報
- [x] 原畫質開機影片
- [x] 多智慧體協作、Office、定時任務
- [x] 38 個預裝技能 + 中文技能市場
- [x] v2.1：本地語音輸入、歷史對話搜尋、電腦操控、全網閱讀、長期記憶、安心模式
- [x] v2.2：本地離線模型、配音與朗讀、創作者工作室、對話統計條
- [x] v2.3：專業軟體操作、增強記憶(可選)
- [ ] 手機遙控(正式版之後)
- [ ] 真機驗收
- [ ] 官方外掛市場換成 cgcode 自己的推薦
- [ ] Claude 客戶端風格介面打磨（毛玻璃等完整 iOS 效果）
- [x] 外掛“發現”頁預設開啟；技能搜尋與一鍵安裝由“技能市場”技能提供
- [x] 自動更新：改用 cgcode 自己的更新源(GitHub Pages)，不再連底座官方伺服器
- [x] 正式版 1.0.0：iOS 藍主按鈕、更大圓角
- [x] 正式版 1.0.0 安裝包(暫未簽名)

---

## 安全審查(1.0.0)

- **自寫程式碼**：改造腳本、建置腳本、更新清單產生器逐一檢查，倉庫中沒有任何金鑰或密碼。
- **用戶端安全設定**：所有視窗都開啟沙箱與上下文隔離，並關閉 Node 整合；外部連結只允許用系統瀏覽器開啟 http/https 位址。
- **更新來源**：自動更新與強制更新策略都只連 cgcode 自己的位址，不連底座官方伺服器；產品遙測與會話上傳已關閉。
- **依賴漏洞**：用 pnpm audit 掃出 37 條已公開漏洞(高危 8 條)。其中隨用戶端發佈的 10 個套件(MCP SDK、hono、body-parser、fast-uri、fflate 等)已強制升級到修復版本；其餘只用於測試或發佈工具，不進安裝包。
- **預裝技能**：38 個技能逐一檢查，沒有發現竊取金鑰、混淆程式碼或越獄內容。需要從網路安裝元件的技能已全部固定版本號，不再「下載腳本直接執行」，安裝前必須先徵得同意。

## 已知限制

- 程式暫未簽名，首次執行會被 Windows SmartScreen 提示，需要手動“仍要執行”。
- 兩個版本都還沒在真實 Windows 電腦上完成驗收。
- 桌面版目前需登入 GitHub 後從 Actions 下載，正式版將釋出到 Releases。
- 部分模型服務（如日日新）有頻率限制，高峰期可能需要等待自動重試。

---

## 常見問題

**必須用日日新嗎？**
不是。任何相容 OpenAI 格式的介面都可以，在嚮導或設定裡改介面地址、Key 和模型名即可。

**開機動畫太長 / 不想要聲音？**
終端版在 `cgcode.toml` 的 `[intro]` 裡把 `enabled` 或 `sound` 改成 `false`；桌面版點選或按任意鍵跳過。

**會不會上傳我的程式碼或對話？**
不會。對話只發給你自己配置的模型服務，所有遙測預設關閉。

**怎麼安裝更多技能？**
桌面版直接對助手說“幫我找個能做 XX 的技能”，會從技能市場搜尋、用中文展示，並在安全檢查後安裝。

---

## 開源許可與致謝

- 終端版基於 [OpenAI Codex](https://github.com/openai/codex)（Apache License 2.0），保留原有 `LICENSE` 與 `NOTICE`。
- 桌面客戶端基於 DeepSeek Harness 構建（MIT License）。cgcode 是獨立產品，與 DeepSeek、OpenAI 無隸屬關係。
- 預裝開源技能來自 [vercel-labs/skills](https://github.com/vercel-labs/skills)（MIT）、[vercel-labs/agent-browser](https://github.com/vercel-labs/agent-browser)（Apache-2.0）、[anthropics/skills](https://github.com/anthropics/skills)（Apache-2.0），各技能目錄內保留原許可證。
