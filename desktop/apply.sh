#!/usr/bin/env bash
# 在底座源码上套用 cgcode 改造。$1=cgcode 仓库根目录，$2=底座源码目录。
set -euo pipefail
ROOT="$1"; SRC="$2"
# 打包配置：不走官方机器预设，使用本地 .env.windows(未签名)。
cat > "$SRC/apps/desktop/.env.windows" <<'EOF'
DSH_DESKTOP_APP_ID=com.cgcode.desktop
DSH_DESKTOP_AUTO_UPDATE_ENV=production
DSH_DESKTOP_MANDATORY_UPDATE_PROD_ORIGIN=https://chengpt2025-hub.github.io
DSH_DESKTOP_WINDOWS_COMPRESSION_LEVEL=3
EOF
echo "apply: 已写入 apps/desktop/.env.windows"
