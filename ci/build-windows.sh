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
export LIBSQLITE3_FLAGS=SQLITE_DISABLE_INTRINSIC
export STABLE_GIT_COMMIT="$(git rev-parse HEAD)"
cd "$ROOT/codex-rs"
rustup show active-toolchain || rustup toolchain install
rustup target add "$TARGET"
cargo build --release --target "$TARGET" --bin codex

# 3. 打包
mkdir -p "$ROOT/dist"
cp "target/$TARGET/release/codex.exe" "$ROOT/dist/cgcode.exe"
echo "== 构建完成：dist/cgcode.exe =="
