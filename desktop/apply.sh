#!/usr/bin/env bash
# 在底座源码上套用 cgcode 改造。$1=cgcode 仓库根目录，$2=底座源码目录。
set -euo pipefail
ROOT="$1"; SRC="$2"
echo "apply: 第一轮先原样构建，验证流水线"
