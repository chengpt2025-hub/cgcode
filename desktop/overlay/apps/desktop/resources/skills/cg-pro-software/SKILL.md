---
name: cg-pro-software
description: "专业软件操作：让助手通过命令行操作 GIMP、Blender、LibreOffice、Krita、Shotcut 等专业软件(批量修图、出 3D 渲染、批量转换文档、剪辑导出等)；用户说“用 Blender…”“批量处理这些图片”“用 LibreOffice 把…转成…”时使用。底层为开源 CLI-Anything(港大 HKUDS，Apache-2.0)。"
---

# 专业软件操作(CLI-Anything)

## 首次使用：安装(需用户同意)
1. 说明：需要 Python 3.10 以上，以及电脑上已安装对应的软件(如 GIMP、Blender、LibreOffice)。
2. 安装软件包管理器：`pip install cli-anything-hub==0.4.1`(或 `uv tool install cli-anything-hub==0.4.1`；固定版本)。
3. 用 `cli-hub search <软件名>` 查找对应工具，`cli-hub install <名称>` 安装。
4. 安装前查看该工具的许可证(CLI-Anything 各子工具许可证标注不完全一致)，非开源许可证的要告诉用户。

## 使用
- 先说明要对哪些文件做什么、输出到哪里，得到同意后执行。
- **不覆盖原文件**：默认输出到新文件夹，例如 `原文件夹\cgcode输出\`。
- 批量操作先拿 1 个文件试做，用户确认效果后再批量。
- 完成后列出生成的文件和用时。

## 规则
- 删除、覆盖文件前必须确认。
- 软件报错时把错误翻译成中文说明原因和解决办法。
