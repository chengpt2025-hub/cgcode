#!/usr/bin/env bash
# cgcode 第二版(桌面客户端)Windows 构建脚本。
# 底座：DeepSeek Harness 开源代码(MIT)，钉在固定提交；在其上套用 desktop/ 下的 cgcode 改造。
set -uo pipefail
ROOT="$(pwd)"
LOG="$ROOT/build.log"
: > "$LOG"
exec > >(tee -a "$LOG") 2>&1
set -e
DSH_SHA="$(cat "$ROOT/desktop/DSH_COMMIT")"
W=/c/cgw
echo "== cgcode v2 构建：底座提交 $DSH_SHA"
rm -rf "$W"; mkdir -p "$W"

echo "== 准备 Node 24"
NV=$(curl -fsSL https://nodejs.org/dist/latest-v24.x/SHASUMS256.txt | grep -o 'node-v[0-9.]*-win-x64\.zip' | head -1)
curl -fsSL "https://nodejs.org/dist/latest-v24.x/$NV" -o "$W/node.zip"
unzip -q "$W/node.zip" -d "$W"
NODE_DIR="$W/${NV%.zip}"
export PATH="$NODE_DIR:$PATH"
# 底座打包脚本按 Windows 自带 tar(bsdtar)写法调用 tar；Git Bash 的 GNU tar 会把 "C:" 当远程主机。
mkdir -p "$W/bin"; cp /c/Windows/System32/tar.exe "$W/bin/tar.exe"
export PATH="$W/bin:$PATH"
tar --version | head -1
node -v
npm i -g pnpm@11.28.5 >/dev/null
pnpm -v

echo "== 获取底座源码"
git config --global core.longpaths true
git init -q "$W/src"
cd "$W/src"
git remote add origin https://github.com/deepseek-ai/deepseek-harness.git
git fetch -q --depth 1 origin "$DSH_SHA"
git checkout -q FETCH_HEAD

echo "== 套用 cgcode 改造"
if [ -d "$ROOT/desktop/overlay" ]; then cp -r "$ROOT/desktop/overlay/." "$W/src/"; fi
if [ -f "$ROOT/desktop/apply.sh" ]; then bash "$ROOT/desktop/apply.sh" "$ROOT" "$W/src"; fi

export DSH_TELEMETRY_DISABLED=1
unset DSH_PACKAGING_PRESET || true
export DSH_DESKTOP_UPDATE_CHANNEL=stable
export PACKAGE_SIGNING=unsigned
export PACKAGE_WINDOWS_COMPRESSION_LEVEL=3
export DSH_DESKTOP_PACKAGE_CACHE_DIR="$W/pkgcache"
export PNPM_CONFIG_STORE_DIR="$W/pnpm-store"

echo "== 安装依赖"
# 安全加固加了 overrides，锁文件需按覆盖重新解析
pnpm install --no-frozen-lockfile

echo "== 检查 PowerShell 7(打包预检需要)"
for d in "/c/Program Files/PowerShell/7" "/c/Program Files/PowerShell/7-preview"; do
  [ -x "$d/pwsh.exe" ] && export PATH="$PATH:$d"
done
if ! command -v pwsh >/dev/null 2>&1; then
  echo "未找到 pwsh，用 winget/dotnet 以外的方式安装 PowerShell 7"
  PWV=7.4.6
  curl -fsSL -o "$W/pwsh.zip" "https://github.com/PowerShell/PowerShell/releases/download/v$PWV/PowerShell-$PWV-win-x64.zip"
  mkdir -p "$W/pwsh" && (cd "$W/pwsh" && unzip -q -o ../pwsh.zip)
  export PATH="$PATH:$W/pwsh"
fi
command -v pwsh
pwsh -NoProfile -NonInteractive -Command '$PSVersionTable.PSVersion.ToString(); Import-Module "$PSHOME/Modules/Microsoft.PowerShell.Security/Microsoft.PowerShell.Security.psd1" -ErrorAction Stop; (Get-Command Get-AuthenticodeSignature).Name' || echo "警告: pwsh 自检失败"

echo "== 打包 Windows 安装程序(未签名)"
pnpm run package:desktop:win:x64:unsigned > "$W/package.log" 2>&1 || { tail -c 30000 "$W/package.log"; exit 1; }
tail -c 8000 "$W/package.log"

echo "== 收集产物"
mkdir -p "$ROOT/dist"
find "$W" -maxdepth 8 \( -iname '*.exe' -o -iname '*.zip' -o -iname '*.blockmap' -o -iname 'latest*.yml' \) -newer "$W/node.zip" \
  -not -path '*/node_modules/*' -not -path '*/pnpm-store/*' -not -path "$NODE_DIR/*" -size +1M 2>/dev/null | tee "$W/found.txt" || true
grep -i 'setup\|installer\|cgcode\|harness' "$W/found.txt" | grep -i '\.exe$' | while read -r f; do cp "$f" "$ROOT/dist/"; done
# 自动更新清单(stable.yml)与差分块文件，供发布到 cgcode 更新源
find "$W" -maxdepth 8 \( -iname 'stable*.yml' -o -iname 'latest*.yml' -o -iname 'nightly*.yml' -o -iname '*.exe.blockmap' \) \
  -not -path '*/node_modules/*' -not -path '*/pnpm-store/*' -newer "$W/node.zip" 2>/dev/null | while read -r f; do cp "$f" "$ROOT/dist/"; done || true
node "$ROOT/desktop/make-update-feed.mjs" "$ROOT/dist" || echo "生成更新清单失败(不影响安装包)"
ls -la "$ROOT/dist"
if ! ls "$ROOT/dist"/*.exe >/dev/null 2>&1; then echo "没有找到安装程序"; exit 1; fi
echo "== 完成"
