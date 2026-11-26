# cgcode 第二版：桌面客户端

## 目标(用户拍板 2026-11-26)
- 发给普通客户使用：安装包签名、自动更新、首次设置零门槛。
- 全能助手：聊天、联网搜索、文档/表格/PPT、画图、写代码。
- 多智能体协作组网(参考 Grok 多智能体)。
- 借鉴 Claude 客户端的体验。
- 多模型，界面切换：日日新默认，可加 DeepSeek、通义、Kimi、OpenAI 兼容等。
- 插件可插拔，预装更多插件和基础技能。

## 底座
DeepSeek Harness(MIT)。遵守品牌规范：产品名为 cgcode，不使用“DeepSeek Harness”作名称，可写“基于 DeepSeek Harness 构建”。
底座提交见 desktop/DSH_COMMIT。

## 构建
v2 分支推送即触发 Windows 构建：ci/build-windows.sh 克隆底座、执行 desktop/apply.sh 与 desktop/overlay，打未签名安装包到 dist/，以 Actions artifact 提供下载。

## 阶段
1. 流水线：原样打包成功。
2. 品牌：名称、图标、开机视频、中文默认、关闭遥测与官方更新源。
3. 模型：预置日日新等多家供应商，界面切换。
4. 插件与技能：预装办公、搜索、画图、编程、多智能体协作等。
5. 体验：Claude 风格界面打磨、首次引导。
6. 发布：签名证书(需用户购买)、自动更新源。
