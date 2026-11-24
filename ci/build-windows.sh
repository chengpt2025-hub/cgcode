#!/usr/bin/env bash
# cgcode Windows 构建脚本（由 .github/workflows/cgcode-build.yml 调用）
# 所有构建逻辑都放在这里，工作流文件本身保持不变。
set -uo pipefail

ROOT="$(pwd)"
LOG="$ROOT/build.log"
exec > >(tee "$LOG") 2>&1
set -e

TARGET="x86_64-pc-windows-msvc"
echo "== cgcode Windows 构建 =="
echo "提交：$(git rev-parse HEAD)"

# 1. 准备 rusty_v8 预编译产物（复用上游 setup-rusty-v8 动作里的脚本）
python -m pip install --quiet pyyaml
V8_SCRIPT="$(mktemp)"
python - "$ROOT/.github/actions/setup-rusty-v8/action.yml" > "$V8_SCRIPT" <<'PY'
import sys, yaml
action = yaml.safe_load(open(sys.argv[1], encoding="utf-8"))
for step in action["runs"]["steps"]:
    if "run" in step:
        print(step["run"])
PY
ENV_FILE="$(mktemp)"
export RUNNER_TEMP="${RUNNER_TEMP:-$(mktemp -d)}"
export GITHUB_WORKSPACE="$ROOT"
export TARGET
GITHUB_ENV="$ENV_FILE" bash "$V8_SCRIPT"
# 把上游脚本写入 GITHUB_ENV 的变量导出到当前环境
while IFS= read -r line; do
  case "$line" in
    *=*) export "${line%%=*}=${line#*=}"; echo "设置环境变量 ${line%%=*}" ;;
  esac
done < "$ENV_FILE"

# 2. 编译
# Windows 默认路径长度上限 260，rusty_v8 的子模块里有超长路径，需要开启长路径并缩短 CARGO_HOME
git config --global core.longpaths true
export CARGO_NET_GIT_FETCH_WITH_CLI=true
export CARGO_HOME="C:/cg"
mkdir -p "$CARGO_HOME"
export LIBSQLITE3_FLAGS=SQLITE_DISABLE_INTRINSIC
export STABLE_GIT_COMMIT="$(git rev-parse HEAD)"
cd "$ROOT/codex-rs"
rustup show active-toolchain || rustup toolchain install
rustup target add "$TARGET"
BINS="codex codex-windows-sandbox-setup codex-command-runner codex-code-mode-host codex-exec-server"
args=()
for b in $BINS; do args+=(--bin "$b"); done
cargo build --release --target "$TARGET" "${args[@]}"

# 3. 打包：cgcode.exe 加上 Windows 下运行命令沙箱需要的配套程序
REL="target/$TARGET/release"
PKG="$ROOT/dist/cgcode-windows-x64"
rm -rf "$ROOT/dist"
mkdir -p "$PKG"
cp "$REL/codex.exe" "$PKG/cgcode.exe"
for b in $BINS; do
  [ "$b" = "codex" ] && continue
  if [ -f "$REL/$b.exe" ]; then cp "$REL/$b.exe" "$PKG/"; else echo "提示：未找到 $b.exe"; fi
done
cp "$ROOT/LICENSE" "$PKG/" 2>/dev/null || true
cp "$ROOT/NOTICE" "$PKG/" 2>/dev/null || true
cat > "$PKG/使用说明.txt" <<'EOF'
cgcode 使用说明

1. 把整个文件夹放到一个固定位置，例如 C:\cgcode
2. 打开“终端”或 PowerShell，进入你的项目目录，运行：
     C:\cgcode\cgcode.exe
   （也可以把 C:\cgcode 加入系统 PATH，之后直接输入 cgcode）
3. 第一次运行会进入中文配置向导，填写接口地址、API Key、模型名即可。
4. 重新配置模型接口：cgcode setup
5. 配置文件位置：%USERPROFILE%\.cgcode\cgcode.toml
EOF
cp "$REL/codex.exe" "$ROOT/dist/cgcode.exe"
(cd "$ROOT/dist" && powershell -NoProfile -Command "Compress-Archive -Path 'cgcode-windows-x64' -DestinationPath 'cgcode-windows-x64.zip' -Force")
rm -rf "$PKG"
ls -la "$ROOT/dist"
echo "== 构建完成 =="
