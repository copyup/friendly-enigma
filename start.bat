@echo off
REM ====================================
REM Rust 博客系统快速启动脚本 (Windows)
REM ====================================

echo.
echo ========================================
echo   Rust 博客系统 - 快速启动
echo ========================================
echo.

REM 检查 Rust 是否安装
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [错误] 未检测到 Rust，请先安装 Rust:
    echo https://rustup.rs/
    pause
    exit /b 1
)

echo [信息] Rust 版本:
rustc --version
echo.

REM 检查 PostgreSQL 是否运行
echo [信息] 检查数据库连接...
psql -U postgres -c "\l" >nul 2>&1
if %errorlevel% neq 0 (
    echo [警告] 无法连接到 PostgreSQL，请确保:
    echo 1. PostgreSQL 服务正在运行
    echo 2. 已创建数据库 blog_db
    echo 3. 已在 .env 文件中配置正确的 DATABASE_URL
    echo.
)

REM 检查 .env 文件
if not exist ".env" (
    echo [信息] 创建 .env 文件...
    copy .env.example .env >nul
    echo [提示] 请编辑 .env 文件，设置正确的 DATABASE_URL
    echo.
    pause
    exit /b 1
)

echo [信息] 开始编译项目...
echo.

REM 编译项目
cargo build

if %errorlevel% neq 0 (
    echo.
    echo [错误] 编译失败，请检查错误信息
    pause
    exit /b 1
)

echo.
echo ========================================
echo   编译成功！
echo ========================================
echo.
echo [提示] 按任意键启动服务器...
pause >nul

echo.
echo [信息] 启动博客服务器...
echo [信息] 访问地址：http://localhost:8080
echo.

REM 运行项目
cargo run
