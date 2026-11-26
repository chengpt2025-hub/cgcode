<div align="center">

<img src="https://raw.githubusercontent.com/chengpt2025-hub/cgcode/v2/desktop/overlay/apps/desktop/resources/icon.png" width="120" alt="cgcode logo" />

# cgcode

### Bring your own model: an AI coding agent and all-in-one desktop assistant built for Chinese users

[简体中文](README.md) | [繁體中文](README.zh-TW.md) | **English** | [日本語](README.ja.md) | [한국어](README.ko.md)

[Download (terminal)](https://github.com/chengpt2025-hub/cgcode/releases) · [Highlights](#highlights) · [Why cgcode](#why-cgcode) · [Roadmap](#roadmap) · [FAQ](#faq)

</div>

---

## In one sentence

cgcode is an **AI assistant that works with any large language model you choose**. It takes two world-class open-source agent foundations (OpenAI Codex and DeepSeek Harness) and rebuilds them into a product that is **fully localized in Chinese, zero-setup, account-free and telemetry-free**, offered as a **terminal edition** for developers and a **desktop client** for everyone else.

| | Terminal edition (v1) | Desktop client (v2) |
|---|---|---|
| For | Developers | Everyone: students, office workers, creators, teams |
| What it does | Understands your project, writes code, edits files, runs commands, reviews code, manages Git | Chat, web search, documents, spreadsheets and slides, image generation, coding, multi-agent collaboration, scheduled tasks |
| How you use it | Run it in a terminal | Install with a double-click; full windowed UI |
| Branch | `main` | `v2` |
| Status | Acceptance build released | Acceptance build available |

---

## Highlights

### 1. Model freedom
- Ships with **SenseNova (日日新)** by SenseTime as the default, ready to use.
- Works with DeepSeek, Qwen, Kimi and any **OpenAI-compatible** endpoint or relay.
- The terminal edition includes a **built-in Chat Completions bridge**. Upstream Codex removed Chat support and only speaks the Responses API; cgcode lets Chat-only models fully drive the agent.
- The desktop client lets you **switch models at any time** from the UI.

### 2. Truly Chinese-first
- The UI, slash-command help, welcome page, status page, key hints and default replies are all in Simplified Chinese.
- Preinstalled skills are written in Chinese and triggered by natural Chinese phrases.

### 3. Zero-barrier setup
- A **Chinese setup wizard** on first launch asks for the endpoint, key and model, then **tests the connection automatically**. No config files to edit.
- The desktop welcome page needs nothing but one API key.
- No ChatGPT, Claude or other overseas account required.

### 4. Privacy first
- Your API key stays on your own computer.
- Conversations go only to the model service you configured.
- Usage reporting, feedback upload, session telemetry and official update checks are **off by default**.

### 5. Reliable
- **Automatic fast retry on rate limits (429)**.
- Optional **fallback endpoints** take over when the primary one fails.

### 6. Skills out of the box
- Terminal edition: **6** Chinese developer skills. Desktop client: **38** skills (19 in-house Chinese skills plus 19 curated popular open-source skills).
- The desktop client includes a **Chinese skill market**: search skills.sh, ClawHub and SkillHub with one sentence, see results in Chinese, and **install with one click after a safety check**.
- Every preinstalled skill is license-checked and security-reviewed. **Jailbreak-style skills are rejected** to protect your account and keys from being banned.

### 7. Beautiful
- **iOS-inspired design**: rounded corners, generous spacing, iOS system colors. The terminal edition has 7 themes with live preview via `/theme`.
- An official **10-second boot animation**: Earth, wormhole, mechanical tunnel, light burst, then CG CODE / SYSTEM ONLINE, with an energetic soundtrack.

### 8. All-in-one with multi-agent (desktop)
- Chat, search, Word / Excel / PowerPoint, images and coding in one app.
- **Multi-agent collaboration** inspired by Grok: complex tasks are split across several agents working in parallel.
- **Scheduled tasks** and **pluggable plugins**.

---

## Why cgcode

| | cgcode | Official Codex CLI | ChatGPT / Claude apps |
|---|---|---|---|
| Models | Any OpenAI-compatible model; SenseNova by default | Mainly OpenAI models; Responses API only | Their own models only |
| Chat-only models (common among Chinese providers) | Supported (built-in bridge) | Not supported | Not supported |
| UI language | Fully Chinese | English | Multilingual |
| Account | None; just your own key | OpenAI account or key | Overseas account |
| First-run setup | Chinese wizard with connection test | Manual configuration | Account sign-in |
| Usage reporting | All off by default | Depends on vendor settings | Depends on vendor policy |
| Preinstalled Chinese skills | 6 (terminal) / 38 + skill market (desktop) | None | None |
| Desktop app for non-developers | Yes | No | Yes |

---

## Quick start

### Terminal edition

1. Download the latest `cgcode-windows-x64.zip` (about 186 MB) from [Releases](https://github.com/chengpt2025-hub/cgcode/releases) and unzip it, for example to `D:\cgcode`.
2. Open Terminal or PowerShell, go to your project folder and run `D:\cgcode\cgcode.exe`.
   - The program is not code-signed yet. If Windows says it protected your PC, click **More info → Run anyway**.
3. Watch the boot animation (press any key to skip), then fill in the wizard:
   - ① Endpoint, e.g. `https://token.sensenova.cn/v1`
   - ② API key
   - ③ Model, e.g. `sensenova-6.8-flash`
4. Once the connection test passes, start chatting.

### Desktop client (acceptance stage)

1. Sign in to GitHub, open [Actions](https://github.com/chengpt2025-hub/cgcode/actions) and pick the latest successful run on the `v2` branch.
2. Under **Artifacts**, download `cgcode-windows-x64` and unzip it.
3. Run `cgcode-<version>-win-x64-unsigned.exe`. If SmartScreen appears, click **More info → Run anyway**.
4. The boot video plays on first launch; then enter your SenseNova API key on the welcome page.

> The official release will ship a signed installer with automatic updates.

---

## Terminal edition in detail

### Themes (7, switch with `/theme`)

| Theme | Notes |
|---|---|
| iOS Dark | Default |
| iOS Light | For light terminal backgrounds |
| Midnight Blue | Deep blue |
| Mint Green | Fresh green |
| Sunset Orange | Warm orange |
| Graphite Gray | Muted gray |
| High Contrast | Accessibility and screen sharing |

Each theme sets both UI colors and syntax highlighting.

### Preinstalled skills (6)

| Skill | What it does | Triggered by |
|---|---|---|
| cg-review | Code review in Chinese, issues grouped by severity with fixes | "review", "check my code" |
| cg-ship | Pre-release check: format, build, test, scan for secrets, write a Chinese commit message | "ship", "pre-commit check" |
| cg-focus | Focus board in `.cgcode/FOCUS.md` to keep long tasks on track | "where are we" |
| cg-memory | Project memory in `.cgcode/MEMORY.md` for conventions and lessons | "remember this" |
| cg-git | Git helper: explains state, handles branches, conflicts and reverts; confirms risky actions | git, branch, conflict |
| cg-context | Context management: summarizes progress, suggests `/compact` or `/new` | "too long", "summarize" |

### Configuration

Location: `%USERPROFILE%\.cgcode\cgcode.toml` (created by the wizard; rarely needs editing).

```toml
[api]
base_url = "https://token.sensenova.cn/v1"
api_key  = "your key"
model    = "sensenova-6.8-flash"

# Optional fallback endpoint used when the primary fails
# [[fallback]]
# base_url = "https://api.deepseek.com/v1"
# api_key  = "..."
# model    = "deepseek-chat"

[intro]
enabled = true   # false disables the boot animation
sound   = true   # false mutes the boot sound
```

---

## Desktop client in detail

### Built-in capabilities
- **Model switching**: SenseNova `sensenova-6.8-flash` by default, with Qwen (qwen-plus, qwen-max) and Kimi (moonshot-v1-128k) preconfigured; add any OpenAI-compatible model.
- **All-in-one**: chat, web search, Word / Excel / PowerPoint, image generation, coding.
- **Multi-agent collaboration**, **scheduled tasks**, **plugin management**.
- Full-quality boot video (1248×704); click or press any key to skip.
- **New in v2.1**:
  - On by default: **local voice input** (SenseVoice, runs on CPU; the model downloads on first use), **conversation history search**, **automatic conversation titles**, **plugin market**.
  - **Computer control**: the assistant can operate other apps on your PC (based on agent-desktop); risky actions are confirmed every time.
  - **Web reach**: read Bilibili, YouTube subtitles, V2EX, Xueqiu, GitHub, RSS and any web page; login-only channels such as Xiaohongshu are off by default, with a ban-risk warning before enabling (based on Agent-Reach).
  - **Long-term memory**: preferences you ask it to remember are stored on your PC and survive new conversations.
  - **Safe mode**: asks before sending, paying or deleting; never handles passwords or codes; treats web content as data, not instructions; reports what it did.
- **New in v2.2**:
  - **Local offline models**: with Ollama installed, a "Local offline models" group appears; no internet or API key needed. Preset: Qwen 3.5 4B, DeepSeek R1 8B/14B.
  - **Voice & narration**: text to speech in many dialects and languages, with voice cloning (own or authorized voices only; files labelled "AI合成"), based on VoxCPM2.
  - **Creator studio**: topic, script, storyboard, character voices and subtitles end to end.
  - **Conversation stats bar**: turns, model and tool time, time to first token.
- **New in v2.3**:
  - **Pro software control**: drive GIMP, Blender, LibreOffice and more through CLI-Anything for batch edits, renders and conversions, without overwriting originals.
  - **Memory plus (optional install)**: claude-mem records what was done and recalls it later; data stays on your PC.
  - Phone remote control needs LAN access and carries security risk, so it moves to after the official release.

### Preinstalled skills (38)

**In-house Chinese skills (19)**

| Skill | What it does |
|---|---|
| cg-writing | Chinese writing and polishing |
| cg-meeting-notes | Meeting minutes |
| cg-data-analysis | Data analysis |
| cg-ppt | Slide decks |
| cg-research | Research |
| cg-code-review | Code review |
| cg-team | Multi-agent task splitting |
| cg-daily-plan | Daily planning |
| cg-skill-market | Chinese skill market: search skills.sh, ClawHub, SkillHub; install after a safety check |
| cg-web-scraper | Web data extraction into Excel |
| cg-multi-search | Multi-engine search (Baidu, Bing, Sogou and more), merged and de-duplicated |
| cg-web-reach | Web reach: Bilibili, YouTube subtitles, V2EX, Xueqiu, GitHub, RSS, web pages |
| cg-desktop-control | Computer control: operate other Windows apps, confirming risky actions |
| cg-long-memory | Long-term memory: keeps preferences you ask it to remember on your PC |
| cg-offline-mode | Offline mode: install Ollama and pick a local model for your GPU |
| cg-voice-studio | Voice & narration: TTS, dialects, voice cloning (authorized only) |
| cg-creator-studio | Creator studio: script → storyboard → voices → subtitles |
| cg-pro-software | Pro software control: GIMP, Blender, LibreOffice and more (CLI-Anything) |
| cg-memory-plus | Memory plus: optional claude-mem install, data stays local |

**Curated open-source skills (19)**

| Skill | What it does | Source / License |
|---|---|---|
| find-skills | Find and install open-source skills | vercel-labs/skills · MIT |
| agent-browser | Browser automation: open pages, click, fill forms, screenshot | vercel-labs/agent-browser · Apache-2.0 |
| frontend-design | Distinctive web pages and interfaces | anthropics/skills · Apache-2.0 |
| skill-creator | Create, test and improve your own skills | anthropics/skills · Apache-2.0 |
| mcp-builder | Build MCP tool servers | anthropics/skills · Apache-2.0 |
| webapp-testing | Test web apps with Playwright | anthropics/skills · Apache-2.0 |
| canvas-design | Posters and visual design | anthropics/skills · Apache-2.0 |
| theme-factory | Color and font themes for docs, slides and pages | anthropics/skills · Apache-2.0 |
| web-artifacts-builder | Complex multi-component web pages | anthropics/skills · Apache-2.0 |
| algorithmic-art | Generative art with p5.js | anthropics/skills · Apache-2.0 |
| pdf | Extract, merge, split, fill and create PDFs | anthropics/skills · Apache-2.0 |
| internal-comms | Status reports, announcements and other internal writing | anthropics/skills · Apache-2.0 |
| humanizer | Remove AI-sounding writing | blader/humanizer · MIT |
| security-audit | Multi-phase code security audit | cloudflare/security-audit-skill · MIT |
| brainstorming | Clarify requirements and options before building | obra/superpowers · MIT |
| writing-plans | Break work into small verifiable steps | obra/superpowers · MIT |
| systematic-debugging | Find the root cause before fixing | obra/superpowers · MIT |
| test-driven-development | Write tests first | obra/superpowers · MIT |
| verification-before-completion | Verify before claiming done | obra/superpowers · MIT |

### Skill safety principles
- Every preinstalled skill is license-checked and its scripts reviewed (no key theft, no suspicious network calls, no system changes).
- Open-source skills are fetched at **pinned commits** at build time to prevent upstream tampering.
- Jailbreak-style skills that bypass model safety limits are **never preinstalled and are filtered from the skill market**: they violate provider terms, can get keys or accounts banned, and carry legal risk when distributed.

---

## Architecture

### Terminal edition

```
Terminal
  └─ cgcode.exe (Rust)
       ├─ Chinese wizard / boot animation / iOS themes / Chinese UI
       ├─ Codex core (files, commands, approvals) ── Responses API
       └─ cgcode-chat-bridge (local bridge, 127.0.0.1)
              └─ Responses ⇄ Chat Completions, rate-limit retry, fallback endpoints
                    └─ Your model API (SenseNova / DeepSeek / Qwen …)
```

| File | Purpose |
|---|---|
| `codex-rs/cgcode-chat-bridge/` | Chat Completions bridge (with unit tests) |
| `codex-rs/cli/src/cgcode.rs` | Config, first-run wizard, preinstalled skills, local bridge startup |
| `codex-rs/cli/src/cgcode_intro.rs` | Terminal boot animation and sound |
| `codex-rs/tui/src/cgcode_theme.rs` | 7 iOS-style themes |
| `.github/workflows/cgcode-build.yml` | Automated Windows build and release |

### Desktop client

Upstream code is not edited directly; customizations are applied at build time, so upstream updates are easy to follow:

```
desktop/
├─ DSH_COMMIT              Pinned upstream version
├─ apply.sh                Applies branding, Chinese default, model config
├─ patch-bundle.mjs        Default model, multi-model setup, telemetry off
├─ patch-desktop.mjs       Welcome-page key, boot video, skill installation
├─ fetch-skills.mjs        Fetches pinned open-source skills and adds Chinese descriptions
├─ upstream-skills.json    Skill manifest (source, version, Chinese description)
└─ overlay/                Icons, boot video, in-house Chinese skills
ci/build-windows.sh        Windows build script (Node 24 + pnpm)
```

### Build and release
- Every push triggers a GitHub Actions build on Windows.
- Terminal edition: about 1.5–2 hours; published to Releases on success.
- Desktop client: about 15–25 minutes; the installer is uploaded as an artifact.
- Each build opens a "build succeeded / build failed" issue with the result.

---

## Roadmap

### Terminal edition
- [x] Chat Completions bridge for Chat-only models
- [x] SenseNova integration, rate-limit retry, fallback endpoints
- [x] Chinese first-run wizard
- [x] Independent branding (cgcode, `~/.cgcode`); ChatGPT login and telemetry removed
- [x] Fully Chinese UI
- [x] 7 iOS-style themes + `/theme`
- [x] Official boot video (terminal character-art version) + boot music
- [x] 6 preinstalled Chinese skills
- [x] Automated Windows build and release
- [ ] Acceptance testing on real hardware
- [ ] Fixes from acceptance feedback; stable v1.0
- [ ] Action log feature

### Desktop client
- [x] cgcode branding, Chinese UI by default
- [x] Multiple models: SenseNova (default), Qwen, Kimi
- [x] Key entry on the welcome page for zero-barrier onboarding
- [x] Session telemetry and product analytics off
- [x] Full-quality boot video
- [x] Multi-agent collaboration, Office, scheduled tasks
- [x] 38 preinstalled skills + Chinese skill market
- [x] v2.1: local voice input, history search, computer control, web reach, long-term memory, safe mode
- [x] v2.2: local offline models, voice & narration, creator studio, stats bar
- [x] v2.3: pro software control, memory plus (optional)
- [ ] Phone remote control (after official release)
- [ ] Acceptance testing on real hardware
- [ ] Replace the upstream plugin market with cgcode recommendations
- [ ] Claude-style UI polish (frosted glass and full iOS effects)
- [ ] Dedicated skill-market UI
- [ ] Code signing and auto-update
- [ ] Official installer release

---

## Known limitations

- The program is not signed yet, so Windows SmartScreen warns on first run; choose "Run anyway".
- Neither edition has completed acceptance testing on a real Windows PC.
- The desktop client currently requires signing in to GitHub to download from Actions; the official release will go to Releases.
- Some providers (such as SenseNova) enforce rate limits; at peak times the automatic retry may take a moment.

---

## FAQ

**Do I have to use SenseNova?**
No. Any OpenAI-compatible endpoint works; change the endpoint, key and model in the wizard or settings.

**The boot animation is too long / I don't want sound.**
Terminal: set `enabled` or `sound` to `false` under `[intro]` in `cgcode.toml`. Desktop: click or press any key to skip.

**Does it upload my code or conversations?**
No. Conversations go only to the model service you configured, and all telemetry is off by default.

**How do I install more skills?**
In the desktop client, just say "find me a skill that can do X". It searches the skill markets, shows results in Chinese and installs after a safety check.

---

## License and acknowledgements

- The terminal edition is based on [OpenAI Codex](https://github.com/openai/codex) (Apache License 2.0); the original `LICENSE` and `NOTICE` are retained.
- The desktop client is built on DeepSeek Harness (MIT License). cgcode is an independent product and is not affiliated with DeepSeek or OpenAI.
- Preinstalled open-source skills come from [vercel-labs/skills](https://github.com/vercel-labs/skills) (MIT), [vercel-labs/agent-browser](https://github.com/vercel-labs/agent-browser) (Apache-2.0) and [anthropics/skills](https://github.com/anthropics/skills) (Apache-2.0); each skill folder keeps its original license.
