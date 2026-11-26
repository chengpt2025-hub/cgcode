<div align="center">

# cgcode

**中文 AI 编程助手 & 全能 AI 桌面助手**

接入你自己的大模型 API · 全中文界面 · 面向 Windows

[下载终端版](https://github.com/chengpt2025-hub/cgcode/releases) · [项目规划](#九项目规划路线图) · [常见问题](#十一常见问题)

</div>

---

## 一、项目简介

cgcode 是一个**面向中文用户的 AI 助手**，分为两个版本：

| | 第一版：终端版 | 第二版：桌面客户端 |
|---|---|---|
| 定位 | 程序员用的 AI 编程助手 | 给普通用户的全能 AI 助手 |
| 运行方式 | 在命令行（终端）里运行 | 像普通软件一样安装，有窗口界面 |
| 能做什么 | 读代码、改代码、执行命令、审查代码、提交 Git | 聊天、联网搜索、写文档、做表格和 PPT、生成图片、编程、多智能体协作、定时任务 |
| 基于 | OpenAI Codex 开源代码（Apache 2.0） | DeepSeek Harness 开源代码（MIT） |
| 代码位置 | `main` 分支 | `v2` 分支 |
| 状态 | 验收版已发布 | 验收版构建中 |

两个版本都**用你自己的大模型 API**（默认商汤日日新 SenseNova），不需要 ChatGPT 账号，不向任何第三方上报使用数据。

---

## 二、主要特点

- **接入任意兼容 OpenAI 格式的模型**：日日新、DeepSeek、通义千问、Kimi，以及各类中转站。终端版内置 Chat Completions 适配层，只支持 Chat 接口的服务也能直接用。
- **全中文**：界面、提示、斜杠命令说明、默认回复都是简体中文。
- **零门槛首次设置**：第一次启动有中文向导，填好 API Key 自动测试连通，不用改配置文件。
- **iOS 风格界面**：圆角、留白、iOS 系统配色，多套主题可切换。
- **开机动画**：官方 10 秒开机视频（地球 → 虫洞 → 机械隧道 → 光爆 → CG CODE / SYSTEM ONLINE），带音乐。
- **预装技能**：开箱即用的中文技能，以及从开源技能市场精选的热门技能，并可在客户端里继续搜索安装。
- **隐私**：默认关闭使用数据上报、反馈上传和官方更新检查。
- **限流自动重试**：遇到模型服务 429 限流时自动快速重试，并支持配置备用接口自动切换。

---

## 三、第一版：终端版（main 分支）

### 3.1 下载与安装

1. 打开 [Releases 页面](https://github.com/chengpt2025-hub/cgcode/releases)，下载最新的 `cgcode-windows-x64.zip`（约 186 MB）。
2. 解压到任意文件夹，例如 `D:\cgcode`。
3. 打开“终端”或“PowerShell”，进入你的项目文件夹，运行 `D:\cgcode\cgcode.exe`。
   - 程序暂未做代码签名，Windows 可能提示“已保护你的电脑”，点击 **更多信息 → 仍要运行**。

### 3.2 首次启动

1. 播放开机动画（按任意键跳过）。
2. 中文设置向导依次询问：
   - ① 接口地址，例如 `https://token.sensenova.cn/v1`
   - ② API Key
   - ③ 模型名，例如 `sensenova-6.8-flash`
3. 向导自动测试连通性，成功后保存，之后直接进入对话。

### 3.3 功能

- 在当前项目目录里读代码、改文件、执行命令；**修改前会征求确认**。
- 中文斜杠命令帮助、欢迎页、`/status` 状态页、底部按键提示。
- `/theme` 实时预览并切换主题（共 7 套）：

| 主题 | 说明 |
|---|---|
| iOS 深色 | 默认 |
| iOS 浅色 | 浅色终端背景适用 |
| 午夜蓝 | 深蓝色调 |
| 薄荷绿 | 清新绿色调 |
| 日落橙 | 暖橙色调 |
| 石墨灰 | 低饱和灰色调 |
| 高对比 | 视力辅助、投屏演示 |

每套主题同时统一界面配色和代码高亮配色。

### 3.4 预装技能（6 个）

| 技能 | 作用 | 怎么触发 |
|---|---|---|
| cg-review | 中文代码审查，按严重程度列问题和修改建议 | “审查”“帮我看看代码” |
| cg-ship | 发布前检查：格式化、编译、测试、查敏感信息，生成中文提交说明 | “发布”“提交前检查” |
| cg-focus | 焦点板：在 `.cgcode/FOCUS.md` 记录目标与进度，长任务不跑偏 | “我们做到哪了” |
| cg-memory | 项目记忆：把约定和踩过的坑记到 `.cgcode/MEMORY.md` | “记住”“以后都这样” |
| cg-git | Git 助手：中文解释状态，处理分支、冲突、回退，危险操作先确认 | 提到 git、分支、冲突 |
| cg-context | 上下文管理：总结进度，建议 `/compact` 或 `/new` | “太长了”“总结一下” |

### 3.5 配置文件

位置：`%USERPROFILE%\.cgcode\cgcode.toml`（向导会自动生成，一般不用手改）。

```toml
[api]
base_url = "https://token.sensenova.cn/v1"
api_key  = "你的 Key"
model    = "sensenova-6.8-flash"

# 可选：备用接口，主接口失败时自动切换
# [[fallback]]
# base_url = "https://api.deepseek.com/v1"
# api_key  = "..."
# model    = "deepseek-chat"

[intro]
# 关闭开机动画或开机声音时改成 false
enabled = true
sound   = true
```

---

## 四、第二版：桌面客户端（v2 分支）

### 4.1 定位

给**普通用户**用的全能 AI 助手：下载安装包、双击安装、填一个 API Key 就能用。

### 4.2 功能

- **多模型切换**：默认日日新 `sensenova-6.8-flash`，预置通义千问（qwen-plus、qwen-max）、Kimi（moonshot-v1-128k），可在界面里添加任意兼容 OpenAI 格式的模型。
- **全能**：聊天、联网搜索、Word / Excel / PPT 文档处理、图片生成、编程。
- **多智能体协作**：参考 Grok 的多智能体模式，复杂任务自动拆分给多个智能体并行完成。
- **定时任务**：让助手按时间自动执行任务。
- **可插拔插件**：插件管理，按需安装、启用、停用。
- **开机动画**：使用官方原画质开机视频（1248×704），点击或按任意键跳过。
- **默认中文**，品牌为 cgcode，关闭会话遥测和产品数据上报。

### 4.3 预装技能（23 个）

**cgcode 自研中文技能（11 个）**

| 技能 | 作用 |
|---|---|
| cg-writing | 中文写作与润色 |
| cg-meeting-notes | 会议纪要整理 |
| cg-data-analysis | 数据分析 |
| cg-ppt | 制作 PPT |
| cg-research | 资料调研 |
| cg-code-review | 代码审查 |
| cg-team | 多智能体分工协作 |
| cg-daily-plan | 每日计划 |
| cg-skill-market | 技能市场：在 skills.sh、ClawHub、国内 SkillHub 搜索技能，中文展示，安全检查后一键安装 |
| cg-web-scraper | 网页数据抓取，整理成 Excel |
| cg-multi-search | 多引擎搜索（百度、必应、搜狗等）汇总去重 |

**开源市场精选技能（12 个）**

| 技能 | 作用 | 来源 / 许可证 |
|---|---|---|
| find-skills | 搜索并安装开源技能 | vercel-labs/skills · MIT |
| agent-browser | 浏览器自动化：打开网页、点击、填表、截图 | vercel-labs/agent-browser · Apache-2.0 |
| frontend-design | 有设计感的网页与界面 | anthropics/skills · Apache-2.0 |
| skill-creator | 创建、测试、改进自己的技能 | anthropics/skills · Apache-2.0 |
| mcp-builder | 开发 MCP 工具服务 | anthropics/skills · Apache-2.0 |
| webapp-testing | 用 Playwright 测试网页 | anthropics/skills · Apache-2.0 |
| canvas-design | 海报与视觉设计 | anthropics/skills · Apache-2.0 |
| theme-factory | 文档、幻灯片、网页配色主题 | anthropics/skills · Apache-2.0 |
| web-artifacts-builder | 多组件复杂网页 | anthropics/skills · Apache-2.0 |
| algorithmic-art | p5.js 生成艺术 | anthropics/skills · Apache-2.0 |
| pdf | PDF 提取、合并拆分、填表、生成 | anthropics/skills · Apache-2.0 |
| internal-comms | 周报、通知等内部文稿 | anthropics/skills · Apache-2.0 |

**技能安全原则**

- 每个预装技能都核对过开源许可证，并检查过附带脚本（无窃取 Key、无可疑联网、无修改系统配置）。
- 开源技能在构建时按**固定提交版本**下载，防止上游被篡改。
- **不预装、并在技能市场中过滤**任何绕过模型安全限制的“破甲 / 越狱”类技能：这类技能违反模型服务条款，可能导致用户的 Key 或账号被封，公开分发也有法律风险。

### 4.4 下载与安装（当前验收阶段）

1. 登录 GitHub，打开 [Actions 页面](https://github.com/chengpt2025-hub/cgcode/actions)，选择 `v2` 分支最新一次成功的构建。
2. 在页面底部 **Artifacts** 下载 `cgcode-windows-x64`，解压。
3. 双击 `cgcode-<版本>-win-x64-unsigned.exe` 安装；出现 SmartScreen 提示时点 **更多信息 → 仍要运行**。
4. 首次启动播放开机视频，然后在欢迎页填入日日新 API Key 即可使用。

> 正式发布后将改为带数字签名的安装包，并支持自动更新。

---

## 五、技术架构

### 5.1 终端版

```
用户终端
  └─ cgcode.exe（Rust）
       ├─ 中文向导 / 开机动画 / iOS 主题 / 中文界面
       ├─ Codex 核心（读写文件、执行命令、审批机制）  ── Responses 接口
       └─ cgcode-chat-bridge（本地适配层，127.0.0.1）
              └─ 把 Responses 请求转换为 Chat Completions，
                 支持限流重试、备用接口切换
                    └─ 你的模型 API（日日新 / DeepSeek / 千问 …）
```

关键代码：

| 文件 | 作用 |
|---|---|
| `codex-rs/cgcode-chat-bridge/` | Chat Completions 适配层（含单元测试） |
| `codex-rs/cli/src/cgcode.rs` | 配置读取、首次向导、预装技能、启动本地适配层 |
| `codex-rs/cli/src/cgcode_intro.rs` | 终端开机动画与声音 |
| `codex-rs/tui/src/cgcode_theme.rs` | 7 套 iOS 风格主题 |
| `.github/workflows/cgcode-build.yml` | Windows 自动构建与发布 |

### 5.2 桌面客户端

不直接修改上游代码，而是在构建时“套用定制”，方便以后跟进上游更新：

```
desktop/
├─ DSH_COMMIT              锁定的上游版本（DeepSeek Harness 4b399c39）
├─ apply.sh                套用品牌、默认中文、模型配置等定制
├─ patch-bundle.mjs        默认模型、多模型接入、关闭遥测
├─ patch-desktop.mjs       欢迎页 Key、开机视频、预装技能安装
├─ fetch-skills.mjs        按固定版本拉取开源技能并补中文说明
├─ upstream-skills.json    开源技能清单（来源、版本、中文说明）
└─ overlay/                图标、开机视频、自研中文技能等资源
ci/build-windows.sh        Windows 构建脚本（Node 24 + pnpm）
```

---

## 六、构建与发布

- 每次推送代码，GitHub Actions 自动在 Windows 上编译。
- 终端版（main）：编译约 1.5–2 小时，成功后自动发布到 Releases。
- 桌面版（v2）：编译约 15–25 分钟，安装包作为 Artifact 上传。
- 每次构建结束会自动开一个“构建成功 / 构建失败”的 Issue 通知结果。

---

## 七、目录速览

| 路径 | 内容 |
|---|---|
| `codex-rs/` | 终端版 Rust 源码（含 cgcode 改造） |
| `desktop/` | 桌面客户端定制与资源（v2 分支） |
| `ci/` | 构建脚本 |
| `PLAN.md` | 改造计划 |
| `LICENSE` / `NOTICE` | 开源许可与版权声明 |

---

## 八、隐私与数据

- 你的 API Key 只保存在本机（终端版：`%USERPROFILE%\.cgcode\cgcode.toml`）。
- 对话内容只发送到你自己配置的模型服务。
- 默认关闭：使用数据上报、反馈上传、会话遥测、官方更新检查。

---

## 九、项目规划（路线图）

### 第一版：终端版
- [x] 导入 OpenAI Codex 上游源码并锁定版本
- [x] Chat Completions 适配层（兼容只支持 Chat 接口的模型）
- [x] 接入日日新，限流自动快速重试，备用接口切换
- [x] 中文首次启动向导
- [x] 品牌独立（cgcode、`~/.cgcode`），移除 ChatGPT 登录与遥测
- [x] 全中文界面（斜杠命令、欢迎页、状态页、底部提示）
- [x] 7 套 iOS 风格主题 + `/theme`
- [x] 官方开机视频（终端字符画版）+ 开机音乐
- [x] 6 个预装中文技能
- [x] Windows 自动构建与发布
- [ ] 在真实 Windows 电脑上完成验收
- [ ] 根据验收反馈修复问题，发布稳定版 v1.0
- [ ] 执行日志功能（记录每次操作，便于回看）

### 第二版：桌面客户端
- [x] 基于 DeepSeek Harness 搭建，品牌改为 cgcode
- [x] 默认中文界面
- [x] 多模型：日日新（默认）、通义千问、Kimi
- [x] 欢迎页填写日日新 Key，零门槛上手
- [x] 关闭会话遥测和产品数据上报
- [x] 原画质开机视频
- [x] 多智能体协作、Office、定时任务（底座自带）
- [x] 23 个预装技能 + 中文技能市场
- [ ] 真机验收
- [ ] 关闭上游官方插件市场指向，换成 cgcode 自己的推荐
- [ ] Claude 客户端风格界面打磨（毛玻璃等完整 iOS 效果）
- [ ] 技能市场独立界面入口（搜索、一键安装、中文说明）
- [ ] 代码签名（需购买证书）
- [ ] 自动更新源
- [ ] 正式版安装包发布

---

## 十、已知限制

- 程序暂未签名，首次运行会被 Windows SmartScreen 提示，需要手动“仍要运行”。
- 两个版本都还没在真实 Windows 电脑上完成验收。
- 桌面版目前只能登录 GitHub 后从 Actions 下载，正式版将发布到 Releases。
- 部分模型服务（如日日新）有频率限制，高峰期可能需要等待自动重试。

---

## 十一、常见问题

**Q：必须用日日新吗？**
不是。任何兼容 OpenAI 格式的接口都可以，在向导或设置里改接口地址、Key 和模型名即可。

**Q：开机动画太长 / 不想要声音？**
终端版：在 `cgcode.toml` 的 `[intro]` 里把 `enabled` 或 `sound` 改成 `false`。桌面版：点击或按任意键跳过。

**Q：会不会上传我的代码或对话？**
不会。对话只发给你自己配置的模型服务，所有遥测默认关闭。

**Q：怎么安装更多技能？**
桌面版直接对助手说“帮我找个能做 XX 的技能”，会从技能市场搜索、用中文展示，并在安全检查后安装。

---

## 十二、开源许可与致谢

- 终端版基于 [OpenAI Codex](https://github.com/openai/codex)（Apache License 2.0），保留原有 `LICENSE` 与 `NOTICE`。
- 桌面客户端基于 DeepSeek Harness 构建（MIT License）。cgcode 是独立产品，与 DeepSeek、OpenAI 无隶属关系。
- 预装开源技能来自 [vercel-labs/skills](https://github.com/vercel-labs/skills)（MIT）、[vercel-labs/agent-browser](https://github.com/vercel-labs/agent-browser)（Apache-2.0）、[anthropics/skills](https://github.com/anthropics/skills)（Apache-2.0），各技能目录内保留原许可证。
