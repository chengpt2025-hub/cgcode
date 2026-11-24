# cgcode 改造计划

cgcode 基于 OpenAI Codex 开源代码（Apache 2.0）改造，目标是一个可以接入任意兼容 OpenAI 格式 API 的中文编程 Agent 终端工具，第一版面向 Windows。

## 第一版（终端版）

1. 导入上游源码，锁定版本 ae21235。
2. 内置 Chat Completions 适配层：上游 Codex 只支持 Responses 接口，cgcode 内置一层转换，让 DeepSeek、通义千问、各类中转站等只支持 Chat Completions 的服务也能直接用。
3. 中文首次启动向导：首次运行时填写接口地址、API Key、模型名，自动测试连通性并保存，不需要手改配置文件。
4. 品牌与独立：程序名 cgcode，配置目录 ~/.cgcode，移除 ChatGPT 登录、遥测上报、OpenAI 云端任务、自动更新检查。
5. 中文化：界面文字、提示信息、默认回复语言为简体中文。
6. 构建发布：GitHub Actions 编译 Windows x64 版本，发布到 Releases，下载即用。

## 验收标准

- 在 Windows 上下载 cgcode.exe，双击或在终端运行即可启动。
- 首次启动按向导填写自己的 API 后能正常对话。
- 能在指定项目目录里读代码、改文件、执行命令，改动前会征求确认。
- 界面和回复为简体中文。

## 第二版（之后）

- 桌面窗口版客户端。
- 更多深度定制功能。
