<div align="center">

<img src="https://raw.githubusercontent.com/chengpt2025-hub/cgcode/v2/desktop/overlay/apps/desktop/resources/icon.png" width="120" alt="cgcode logo" />

# cgcode

### 用你自己的大模型，打造中文世界的 AI 编程助手与全能桌面助手

**简体中文** | [繁體中文](README.zh-TW.md) | [English](README.en.md) | [日本語](README.ja.md) | [한국어](README.ko.md)

[下载终端版](https://github.com/chengpt2025-hub/cgcode/releases) · [核心亮点](#核心亮点) · [为什么选择-cgcode](#为什么选择-cgcode) · [路线图](#路线图) · [常见问题](#常见问题)

</div>

---

## 一句话介绍

cgcode 是一个**面向中文用户、可接入任意大模型的 AI 助手**。它把世界一流的开源 AI Agent 底座（OpenAI Codex、DeepSeek Harness）深度改造成**全中文、零门槛、不绑定账号、不上报数据**的产品，同时提供程序员用的**终端版**和普通人用的**桌面客户端**。

| | 终端版（第一版） | 桌面客户端（第二版） |
|---|---|---|
| 适合谁 | 程序员、开发者 | 所有人：学生、上班族、创作者、团队 |
| 能做什么 | 读懂项目、写代码、改文件、跑命令、审查代码、管理 Git | 聊天、联网搜索、写文档、做表格和 PPT、生成图片、编程、多智能体协作、定时任务 |
| 使用方式 | 在终端里运行 | 双击安装，有完整窗口界面 |
| 代码分支 | `main` | `v2` |
| 当前状态 | 验收版已发布 | 验收版已可下载 |

---

## 核心亮点

### 1. 模型自由：想用哪个就用哪个
- 默认接入商汤**日日新 SenseNova**，开箱即用。
- 支持 DeepSeek、通义千问、Kimi 以及任何**兼容 OpenAI 格式**的接口和中转站。
- 终端版**独家内置 Chat Completions 适配层**：上游 Codex 已移除 Chat 接口、只支持 Responses 接口，cgcode 让只支持 Chat 接口的国产模型也能完整驱动 Agent。
- 桌面版可在界面里**随时切换模型**。

### 2. 真正的全中文
- 界面、斜杠命令说明、欢迎页、状态页、底部提示、默认回复，全部是简体中文。
- 预装技能用中文编写，按中文说法触发（例如“帮我审查代码”“提交前检查”）。

### 3. 零门槛上手
- 第一次启动有**中文设置向导**：填接口地址、Key、模型名，**自动测试连通**，不用改任何配置文件。
- 桌面版欢迎页只要填一个 API Key 就能开始用。
- 不需要 ChatGPT、Claude 等海外账号。

### 4. 隐私优先
- API Key 只保存在你自己的电脑上。
- 对话只发送到你自己配置的模型服务。
- **默认关闭**使用数据上报、反馈上传、会话遥测和官方更新检查。

### 5. 稳定耐用
- 遇到模型服务**限流（429）自动快速重试**。
- 可配置**备用接口**，主接口失败时自动切换。

### 6. 开箱即用的技能生态
- 终端版预装 **6 个**中文开发技能；桌面版预装 **38 个**技能（19 个自研中文技能 + 19 个开源市场精选热门技能）。
- 桌面版内置**中文技能市场**：一句话搜索 skills.sh、ClawHub、国内 SkillHub，结果中文展示，**安全检查后一键安装**。
- 每个预装技能都核对过开源许可证并做过安全审查，**拒绝“破甲 / 越狱”类技能**，保护你的账号和 Key 不被封。

### 7. 好看
- **iOS 风格设计**：圆角、留白、iOS 系统配色；终端版 7 套主题，用 `/theme` 实时预览切换。
- 官方 **10 秒开机动画**：地球 → 虫洞 → 机械隧道 → 光爆 → CG CODE / SYSTEM ONLINE，配激昂音乐。

### 8. 全能 + 多智能体（桌面版）
- 一个应用搞定聊天、搜索、Word / Excel / PPT、图片、编程。
- 参考 Grok 的**多智能体协作**：复杂任务自动拆分给多个智能体并行完成。
- **定时任务**、**可插拔插件**，按需扩展。

---

## 为什么选择 cgcode

| 对比项 | cgcode | 官方 Codex CLI | ChatGPT / Claude 等海外客户端 |
|---|---|---|---|
| 可用模型 | 任意兼容 OpenAI 格式的模型，默认国产日日新 | 主要面向 OpenAI 模型，仅支持 Responses 接口 | 只能用自家模型 |
| 只支持 Chat 接口的国产模型 | 支持（内置适配层） | 不支持 | 不支持 |
| 界面语言 | 全中文 | 英文 | 多语言 |
| 账号要求 | 不需要，填自己的 Key 即可 | 需要 OpenAI 账号或 Key | 需要海外账号 |
| 首次设置 | 中文向导，自动测连通 | 手动配置 | 登录账号 |
| 使用数据上报 | 默认全部关闭 | 视官方设置而定 | 视官方政策而定 |
| 预装中文技能 | 终端版 6 个 / 桌面版 38 个 + 中文技能市场 | 无 | 无 |
| 普通人能用的桌面版 | 有 | 无 | 有 |

---

## 快速开始

### 终端版

1. 打开 [Releases 页面](https://github.com/chengpt2025-hub/cgcode/releases)，下载最新的 `cgcode-windows-x64.zip`（约 186 MB），解压到任意文件夹，例如 `D:\cgcode`。
2. 打开“终端”或“PowerShell”，进入你的项目文件夹，运行 `D:\cgcode\cgcode.exe`。
   - 程序暂未做代码签名，若提示“Windows 已保护你的电脑”，点击 **更多信息 → 仍要运行**。
3. 观看开机动画（按任意键跳过），按中文向导填入：
   - ① 接口地址，例如 `https://token.sensenova.cn/v1`
   - ② API Key
   - ③ 模型名，例如 `sensenova-6.8-flash`
4. 连通测试通过后直接开始对话。

### 桌面客户端（验收阶段）

1. 登录 GitHub，打开 [Actions 页面](https://github.com/chengpt2025-hub/cgcode/actions)，选择 `v2` 分支最新一次成功的构建。
2. 在页面底部 **Artifacts** 下载 `cgcode-windows-x64` 并解压。
3. 双击 `cgcode-<版本>-win-x64-unsigned.exe` 安装；出现 SmartScreen 提示时点 **更多信息 → 仍要运行**。
4. 首次启动播放开机视频，然后在欢迎页填入日日新 API Key 即可使用。

> 正式版将提供带数字签名的安装包，并支持自动更新。

---

## 终端版详解

### 主题（7 套，`/theme` 切换）

| 主题 | 说明 |
|---|---|
| iOS 深色 | 默认 |
| iOS 浅色 | 适合浅色终端背景 |
| 午夜蓝 | 深蓝色调 |
| 薄荷绿 | 清新绿色调 |
| 日落橙 | 暖橙色调 |
| 石墨灰 | 低饱和灰色调 |
| 高对比 | 视力辅助、投屏演示 |

每套主题同时统一界面配色与代码高亮配色。

### 预装技能（6 个）

| 技能 | 作用 | 怎么触发 |
|---|---|---|
| cg-review | 中文代码审查，按严重程度列出问题和修改建议 | “审查”“帮我看看代码” |
| cg-ship | 发布前检查：格式化、编译、测试、查敏感信息，生成中文提交说明 | “发布”“提交前检查” |
| cg-focus | 焦点板：在 `.cgcode/FOCUS.md` 记录目标与进度，长任务不跑偏 | “我们做到哪了” |
| cg-memory | 项目记忆：把约定和踩过的坑记到 `.cgcode/MEMORY.md` | “记住”“以后都这样” |
| cg-git | Git 助手：中文解释状态，处理分支、冲突、回退，危险操作先确认 | 提到 git、分支、冲突 |
| cg-context | 上下文管理：总结进度，建议 `/compact` 或 `/new` | “太长了”“总结一下” |

### 配置文件

位置：`%USERPROFILE%\.cgcode\cgcode.toml`（向导自动生成，一般不用手改）。

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
enabled = true   # 改成 false 关闭开机动画
sound   = true   # 改成 false 关闭开机声音
```

---

## 桌面客户端详解

### 内置能力
- **多模型切换**：默认日日新 `sensenova-6.8-flash`，预置通义千问（qwen-plus、qwen-max）、Kimi（moonshot-v1-128k），可添加任意兼容 OpenAI 格式的模型。
- **全能**：聊天、联网搜索、Word / Excel / PPT、图片生成、编程。
- **多智能体协作**、**定时任务**、**插件管理**。
- 原画质开机视频（1248×704），点击或按任意键跳过。
- **v2.1 新增**：
  - 默认开启**本地语音输入**(SenseVoice，CPU 即可运行，首次使用需下载模型)、**历史对话搜索**、**对话自动标题**、**插件市场**。
  - **电脑操控**：让助手操作电脑上的其他软件(基于 agent-desktop)，危险操作每次先确认。
  - **全网阅读**：读 B站、YouTube 字幕、V2EX、雪球、GitHub、RSS 和任意网页；小红书等需登录的渠道默认关闭，开启前提示用小号防封号(基于 Agent-Reach)。
  - **长期记忆**：你明确让它记住的偏好保存在本机，换对话也记得。
  - **安心模式**：发消息、付款、删除等操作先征得同意；不碰密码和验证码；网页里的文字只当资料、不当指令；做完汇报做了什么。
- **v2.2 新增**：
  - **本地离线模型**：装了 Ollama 后模型列表出现“本地离线模型”，不联网、不需要 API Key，预置通义千问 3.5 4B、DeepSeek R1 8B/14B。
  - **配音与朗读**：文字转语音，支持多种方言和外语，可克隆声音(须本人或已获授权，文件名带“AI合成”)，基于 VoxCPM2。
  - **创作者工作室**：选题、剧本、分镜、角色配音、字幕一条龙。
  - **对话统计条**：显示轮次、模型与工具耗时、首字等待时间。
- **v2.3 新增**：
  - **专业软件操作**：通过 CLI-Anything 操作 GIMP、Blender、LibreOffice 等专业软件，批量修图、渲染、转文档，不覆盖原文件。
  - **增强记忆(可选安装)**：claude-mem 自动记录做过的事并在以后自动回忆，数据只存本机。
  - 手机遥控需要开放局域网访问，安全风险较大，移到正式版之后再做。

### 预装技能（38 个）

**cgcode 自研中文技能（19 个）**

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
| cg-skill-market | 中文技能市场：搜索 skills.sh、ClawHub、SkillHub，安全检查后一键安装 |
| cg-web-scraper | 网页数据抓取，整理成 Excel |
| cg-multi-search | 多引擎搜索（百度、必应、搜狗等）汇总去重 |
| cg-web-reach | 全网阅读：B站、YouTube 字幕、V2EX、雪球、GitHub、RSS、网页等 |
| cg-desktop-control | 电脑操控：操作 Windows 上的其他软件，危险操作先确认 |
| cg-long-memory | 长期记忆：把你要求记住的偏好保存在本机 |
| cg-offline-mode | 离线模式：安装 Ollama 并按显存推荐本地模型 |
| cg-voice-studio | 配音与朗读：文字转语音、方言、声音克隆(须授权) |
| cg-creator-studio | 创作者工作室：剧本→分镜→配音→字幕 |
| cg-pro-software | 专业软件操作：GIMP、Blender、LibreOffice 等(CLI-Anything) |
| cg-memory-plus | 增强记忆：可选安装 claude-mem，数据只存本机 |

**开源市场精选技能（19 个）**

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
| humanizer | 去 AI 味：让文字更像真人写的 | blader/humanizer · MIT |
| security-audit | 分阶段代码安全审计 | cloudflare/security-audit-skill · MIT |
| brainstorming | 动手前先把需求问清楚、列出方案 | obra/superpowers · MIT |
| writing-plans | 把任务拆成可执行、可验证的小步骤 | obra/superpowers · MIT |
| systematic-debugging | 系统化排错，先找根因 | obra/superpowers · MIT |
| test-driven-development | 先写测试再写代码 | obra/superpowers · MIT |
| verification-before-completion | 声称完成前必须实际验证 | obra/superpowers · MIT |

### 技能安全原则
- 每个预装技能都核对过开源许可证，并检查过附带脚本（无窃取 Key、无可疑联网、无修改系统配置）。
- 开源技能在构建时按**固定提交版本**下载，防止上游被篡改。
- **不预装、并在技能市场中过滤**绕过模型安全限制的“破甲 / 越狱”类技能：这类技能违反模型服务条款，可能导致 Key 或账号被封，公开分发也有法律风险。

---

## 技术架构

### 终端版

```
用户终端
  └─ cgcode.exe（Rust）
       ├─ 中文向导 / 开机动画 / iOS 主题 / 中文界面
       ├─ Codex 核心（读写文件、执行命令、审批机制）── Responses 接口
       └─ cgcode-chat-bridge（本地适配层，127.0.0.1）
              └─ Responses ⇄ Chat Completions 转换，限流重试，备用接口切换
                    └─ 你的模型 API（日日新 / DeepSeek / 千问 …）
```

| 文件 | 作用 |
|---|---|
| `codex-rs/cgcode-chat-bridge/` | Chat Completions 适配层（含单元测试） |
| `codex-rs/cli/src/cgcode.rs` | 配置、首次向导、预装技能、启动本地适配层 |
| `codex-rs/cli/src/cgcode_intro.rs` | 终端开机动画与声音 |
| `codex-rs/tui/src/cgcode_theme.rs` | 7 套 iOS 风格主题 |
| `.github/workflows/cgcode-build.yml` | Windows 自动构建与发布 |

### 桌面客户端

不直接改上游代码，而是构建时“套用定制”，方便持续跟进上游更新：

```
desktop/
├─ DSH_COMMIT              锁定的上游版本
├─ apply.sh                套用品牌、默认中文、模型配置等定制
├─ patch-bundle.mjs        默认模型、多模型接入、关闭遥测
├─ patch-desktop.mjs       欢迎页 Key、开机视频、预装技能安装
├─ fetch-skills.mjs        按固定版本拉取开源技能并补中文说明
├─ upstream-skills.json    开源技能清单（来源、版本、中文说明）
└─ overlay/                图标、开机视频、自研中文技能等资源
ci/build-windows.sh        Windows 构建脚本（Node 24 + pnpm）
```

### 构建与发布
- 每次推送代码，GitHub Actions 自动在 Windows 上编译。
- 终端版：约 1.5–2 小时，成功后自动发布到 Releases。
- 桌面版：约 15–25 分钟，安装包作为 Artifact 上传。
- 每次构建结束自动开一个“构建成功 / 构建失败”的 Issue 通知结果。

---

## 路线图

### 终端版
- [x] Chat Completions 适配层（兼容只支持 Chat 接口的模型）
- [x] 接入日日新，限流自动重试，备用接口切换
- [x] 中文首次启动向导
- [x] 品牌独立（cgcode、`~/.cgcode`），移除 ChatGPT 登录与遥测
- [x] 全中文界面
- [x] 7 套 iOS 风格主题 + `/theme`
- [x] 官方开机视频（终端字符画版）+ 开机音乐
- [x] 6 个预装中文技能
- [x] Windows 自动构建与发布
- [ ] 真机验收
- [ ] 根据验收反馈修复，发布稳定版 v1.0
- [ ] 执行日志功能

### 桌面客户端
- [x] 品牌改为 cgcode，默认中文界面
- [x] 多模型：日日新（默认）、通义千问、Kimi
- [x] 欢迎页填 Key，零门槛上手
- [x] 关闭会话遥测和产品数据上报
- [x] 原画质开机视频
- [x] 多智能体协作、Office、定时任务
- [x] 38 个预装技能 + 中文技能市场
- [x] v2.1：本地语音输入、历史对话搜索、电脑操控、全网阅读、长期记忆、安心模式
- [x] v2.2：本地离线模型、配音与朗读、创作者工作室、对话统计条
- [x] v2.3：专业软件操作、增强记忆(可选)
- [ ] 手机遥控(正式版之后)
- [ ] 真机验收
- [ ] 官方插件市场换成 cgcode 自己的推荐
- [ ] Claude 客户端风格界面打磨（毛玻璃等完整 iOS 效果）
- [ ] 技能市场独立界面入口
- [ ] 代码签名与自动更新
- [ ] 正式版安装包发布

---

## 已知限制

- 程序暂未签名，首次运行会被 Windows SmartScreen 提示，需要手动“仍要运行”。
- 两个版本都还没在真实 Windows 电脑上完成验收。
- 桌面版目前需登录 GitHub 后从 Actions 下载，正式版将发布到 Releases。
- 部分模型服务（如日日新）有频率限制，高峰期可能需要等待自动重试。

---

## 常见问题

**必须用日日新吗？**
不是。任何兼容 OpenAI 格式的接口都可以，在向导或设置里改接口地址、Key 和模型名即可。

**开机动画太长 / 不想要声音？**
终端版在 `cgcode.toml` 的 `[intro]` 里把 `enabled` 或 `sound` 改成 `false`；桌面版点击或按任意键跳过。

**会不会上传我的代码或对话？**
不会。对话只发给你自己配置的模型服务，所有遥测默认关闭。

**怎么安装更多技能？**
桌面版直接对助手说“帮我找个能做 XX 的技能”，会从技能市场搜索、用中文展示，并在安全检查后安装。

---

## 开源许可与致谢

- 终端版基于 [OpenAI Codex](https://github.com/openai/codex)（Apache License 2.0），保留原有 `LICENSE` 与 `NOTICE`。
- 桌面客户端基于 DeepSeek Harness 构建（MIT License）。cgcode 是独立产品，与 DeepSeek、OpenAI 无隶属关系。
- 预装开源技能来自 [vercel-labs/skills](https://github.com/vercel-labs/skills)（MIT）、[vercel-labs/agent-browser](https://github.com/vercel-labs/agent-browser)（Apache-2.0）、[anthropics/skills](https://github.com/anthropics/skills)（Apache-2.0），各技能目录内保留原许可证。
