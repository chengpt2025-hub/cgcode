#!/usr/bin/env bash
# 在底座源码上套用 cgcode 改造。$1=cgcode 仓库根目录，$2=底座源码目录。
set -euo pipefail
ROOT="$1"; SRC="$2"
D="$SRC/apps/desktop"

# 1. 打包配置：本地未签名 .env.windows
cat > "$D/.env.windows" <<'EOF'
DSH_DESKTOP_APP_ID=com.cgcode.desktop
DSH_DESKTOP_AUTO_UPDATE_ENV=production
DSH_DESKTOP_MANDATORY_UPDATE_PROD_ORIGIN=https://chengpt2025-hub.github.io
DSH_DESKTOP_WINDOWS_COMPRESSION_LEVEL=3
EOF

# 2. 品牌：产品名、协议名、安装包文件名、关于面板(遵守品牌规范，名称不使用 DeepSeek Harness)
B="$D/scripts/electron-builder-config.mjs"
sed -i "s/productName: 'DeepSeek Harness'/productName: 'cgcode'/" "$B"
sed -i "s/protocols: \[{ name: 'DeepSeek Harness', schemes: \['dsh'\] }\]/protocols: [{ name: 'cgcode', schemes: ['cgcode'] }]/" "$B"
sed -i 's/artifactName: `deepseek-harness-/artifactName: `cgcode-/' "$B"
sed -i "s/NSMicrophoneUsageDescription: 'DeepSeek Harness uses/NSMicrophoneUsageDescription: 'cgcode uses/" "$B"
sed -i "s/applicationName: 'DeepSeek Harness'/applicationName: 'cgcode'/" "$D/src/main.ts"
# 桌面壳界面文案中的产品名
sed -i "s/'DeepSeek Harness'/'cgcode'/g" "$D/src/locale.ts"

# 2b. 打包脚本、安装器文案、卸载登记里写死的产品名/可执行文件名，统一改为 cgcode
grep -rlZ "DeepSeek Harness" "$D/scripts" "$D/installer" "$D/src" --include=*.ts --include=*.mjs --include=*.ps1 --include=*.h --include=*.json --include=*.nsh 2>/dev/null \
  | xargs -0 -r sed -i 's/DeepSeek Harness/cgcode/g'
echo "剩余写死产品名的文件数: $(grep -rl "DeepSeek Harness" "$D/scripts" "$D/installer" "$D/src" --include=*.ts --include=*.mjs --include=*.ps1 --include=*.h --include=*.json --include=*.nsh 2>/dev/null | wc -l)"

# 3. 默认中文：没有手动选语言时优先简体中文
sed -i 's/for (const language of languages) {/for (const language of ["zh", ...languages]) {/' "$D/src/locale.ts"

grep -c "productName: 'cgcode'" "$B"
grep -c '\["zh", ...languages\]' "$D/src/locale.ts"
# 4. 模型与遥测：默认日日新、预置通义/Kimi(DeepSeek 由底座自带路由提供)、关闭桌面产品遥测
node "$ROOT/desktop/patch-bundle.mjs" "$SRC"
# 5. 模型插件文案里的产品名
grep -rlZ "DeepSeek Harness" "$SRC/packages" --include=*.json 2>/dev/null | grep -z "/locale/" | xargs -0 -r sed -i 's/DeepSeek Harness/cgcode/g'
# 6. 首次欢迎页：填写日日新 API Key
node "$ROOT/desktop/patch-desktop.mjs" "$SRC"
echo "apply: 品牌与中文默认已套用"
