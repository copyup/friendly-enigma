#!/bin/bash

# ====================================
# Rust 博客系统快速启动脚本 (Linux/Mac)
# ====================================

echo ""
echo "========================================"
echo "  Rust 博客系统 - 快速启动"
echo "========================================"
echo ""

# 检查 Rust 是否安装
if ! command -v rustc &> /dev/null; then
    echo "[错误] 未检测到 Rust，请先安装 Rust:"
    echo "https://rustup.rs/"
    exit 1
fi

echo "[信息] Rust 版本:"
rustc --version
echo ""

# 检查 .env 文件
if [ ! -f ".env" ]; then
    echo "[信息] 创建 .env 文件..."
    cp .env.example .env
    echo "[提示] 请编辑 .env 文件，设置正确的 DATABASE_URL"
    exit 1
fi

echo "[信息] 开始编译项目..."
echo ""

# 编译项目
cargo build

if [ $? -ne 0 ]; then
    echo ""
    echo "[错误] 编译失败，请检查错误信息"
    exit 1
fi

echo ""
echo "========================================"
echo "  编译成功！"
echo "========================================"
echo ""
echo "[信息] 启动博客服务器..."
echo "[信息] 访问地址：http://localhost:8080"
echo ""

# 运行项目
cargo run
