---
name: cg-offline-mode
description: "离线模式：没有网络、不想花 token 费或处理涉密文件时，指导安装 Ollama 并在本机运行大模型，按显存推荐合适的模型；用户说“离线用”“本地模型”“不联网”时使用。"
---

# 离线模式(本地模型)

## 检查
1. 运行 `ollama --version`；能输出版本号说明已安装。
2. 运行 `ollama list` 查看已下载的模型。

## 安装 Ollama(需用户同意)
- 告诉用户：“我要安装 Ollama，它能在你电脑上运行大模型，安装不需要管理员权限，模型另外下载，几 GB 到十几 GB。”
- 同意后运行 `winget install Ollama.Ollama`；没有 winget 时请用户到 https://ollama.com/download 下载 Windows 安装包。

## 按显存选模型
先运行 `nvidia-smi` 查看显存(没有 NVIDIA 显卡则按内存，速度会慢)：
| 可用显存 | 推荐模型 | 下载命令 |
|---|---|---|
| 约 8 GB | 通义千问 3.5 4B 或 DeepSeek R1 8B | `ollama pull qwen3.5:4b` / `ollama pull deepseek-r1:8b` |
| 约 16 GB | DeepSeek R1 14B | `ollama pull deepseek-r1:14b` |
下载前说明大小并征得同意。

## 使用
- 下载完成后，在模型选择器里选“本地离线模型(Ollama)”下的模型即可，不需要 API Key。
- 离线模型比云端模型能力弱、速度取决于电脑配置，复杂任务建议切回云端模型。
- 处理涉密文件时提醒用户切换到本地模型，内容不会离开电脑。
