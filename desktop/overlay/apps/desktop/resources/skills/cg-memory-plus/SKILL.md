---
name: cg-memory-plus
description: "增强记忆(可选安装)：安装开源 claude-mem，让助手自动记录每次做过的事并在以后的对话里自动回忆，比“长期记忆”更全面；用户说“装增强记忆”“让你自动记住做过的事”时使用。claude-mem 为 Apache-2.0 开源项目，数据只存本机。"
---

# 增强记忆(claude-mem，可选)

## 先说明再安装
告诉用户：
- 它会在后台运行一个小程序，自动把每次对话做过的事压缩成记忆，下次自动想起来。
- 记忆只保存在这台电脑上；压缩记忆时会调用当前使用的模型，产生少量 token 费用。
- 需要 Node.js 18 以上；随时可以卸载。
用户同意后再安装。安装过程可能自动下载 Bun、uv 运行时并在本机启动一个后台程序，安装前要把这点告诉用户。

## 安装
1. 检查 `node --version`，没有就征得同意后用 `winget install OpenJS.NodeJS.LTS` 安装。
2. 运行 `npx -y claude-mem@17.0.1 install --ide dsh --dsh-profile desktop --runtime worker`。
3. 运行 `npx -y claude-mem@17.0.1 doctor` 检查后台程序是否正常；如提示 profile 不存在，按提示列出的 profile 名重试。
4. 完成后请用户重启 cgcode。

## 使用
- 安装后会多出记忆搜索类工具(搜索、时间线、查看详情、保存)。用户问“我们之前做过……吗”时先用它们查。

## 卸载
运行 `npx -y claude-mem@17.0.1 uninstall`，并告诉用户已删除哪些内容。

## 不要记
密码、验证码、API Key 等敏感信息出现时提醒用户，不要主动保存。
