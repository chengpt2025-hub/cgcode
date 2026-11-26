<div align="center">

<img src="https://raw.githubusercontent.com/chengpt2025-hub/cgcode/v2/desktop/overlay/apps/desktop/resources/icon.png" width="120" alt="cgcode logo" />

# cgcode

[简体中文](README.md) | [繁體中文](README.zh-TW.md) | **English** | [日本語](README.ja.md) | [한국어](README.ko.md)

</div>

## Pros

- **Any model**: SenseNova by default, plus DeepSeek, Qwen, Kimi and any OpenAI-compatible API or relay. The terminal edition has a built-in Chat Completions adapter, so models that only support the Chat API can fully drive the agent. The desktop client switches models from the UI.
- **Fully Chinese**: UI, command help, welcome page, hints and preinstalled skills are all in Simplified Chinese.
- **Zero setup barrier**: a Chinese setup wizard tests the connection on first launch; the desktop client needs only one API key; no overseas account required.
- **Privacy first**: keys stay on your machine and chats go only to the model you configure; usage reporting and session telemetry are off by default; auto-update contacts only cgcode's own address.
- **Reliable**: automatic retry on rate limits, optional fallback endpoint.
- **Rich skills**: 6 preinstalled in the terminal edition and 38 in the desktop client, plus a Chinese skill market with security-checked one-click install; no jailbreak skills.
- **All-in-one** (desktop): chat, web search, Word/Excel/PPT, images, coding, multi-agent collaboration, scheduled tasks, computer control, web reading, long-term memory, local offline models, voice, creator studio.
- **Secure**: every window is sandboxed; vulnerable dependencies shipped with the client are upgraded to patched versions; skills that install components pin exact versions and ask before installing.
- **Good-looking**: iOS-style UI (iOS blue, large corner radii), 7 terminal themes, a 10-second intro animation with music.

## Cons

- Windows only.
- Not yet accepted on real Windows hardware.
- The installer is unsigned, so Windows warns on first run; choose "Run anyway".
- The desktop installer is about 340 MB and currently has to be downloaded from GitHub Actions while signed in.
- Auto-update works only after GitHub Pages is enabled for the repository and the update files are published.
- Some features install extra components on first use (Node.js, uv, Ollama); voice needs Python and a capable NVIDIA GPU.
- Reading social platforms that require login is off by default and carries a ban risk when enabled.
- Frosted glass and other full iOS effects, and phone remote control, are not available yet.
- Model services such as SenseNova are rate-limited; at peak times requests wait for automatic retry.
