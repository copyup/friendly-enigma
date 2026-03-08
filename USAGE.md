# 博客系统使用指南

## 快速开始

### 第一步：准备环境

1. 安装 Rust (1.70+)
   ```bash
   # Windows PowerShell
   winget install Rustlang.Rust.MSVC
   
   # 或使用 rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. 安装 PostgreSQL (12+)
   - 下载地址：https://www.postgresql.org/download/
   - 安装后记住超级用户密码

### 第二步：配置数据库

1. 打开 pgAdmin 或 psql 命令行工具

2. 执行数据库创建命令：
   ```sql
   CREATE DATABASE blog_db;
   ```

3. 复制 `.env.example` 为 `.env`：
   ```bash
   cp .env.example .env
   ```

4. 编辑 `.env` 文件，修改数据库连接字符串：
   ```
   DATABASE_URL=postgres://postgres:你的密码@localhost:5432/blog_db
   ```

### 第三步：运行项目

1. 编译并运行：
   ```bash
   cargo run
   ```

2. 首次运行时会自动：
   - 连接数据库
   - 执行数据库迁移（创建表结构）
   - 启动 Web 服务器

3. 访问 http://localhost:8080 查看博客首页

## 管理博客内容

### 方式一：使用 RESTful API

#### 1. 创建分类

```bash
curl -X POST http://localhost:8080/api/categories \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"技术\",\"slug\":\"tech\",\"description\":\"技术文章\"}"
```

#### 2. 创建文章

```bash
curl -X POST http://localhost:8080/api/posts \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"我的第一篇文章\",\"slug\":\"first-post\",\"content\":\"# Hello\\n\\n这是内容\",\"status\":\"published\"}"
```

#### 3. 创建页面

```bash
curl -X POST http://localhost:8080/api/pages \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"关于我\",\"slug\":\"about\",\"content\":\"# About\\n\\n这是我的介绍\",\"is_published\":true}"
```

### 方式二：使用 MCP 接口（推荐 AI Agent 使用）

MCP (Model Context Protocol) 是统一的 AI 操作接口。

#### MCP 操作类型

- `1` - Create（创建）
- `2` - Read（读取）
- `3` - Update（更新）
- `4` - Delete（删除）
- `5` - List（列表）

#### MCP 资源类型

- `"post"` - 文章
- `"page"` - 页面
- `"category"` - 分类

#### 示例

**创建文章：**
```bash
curl -X POST http://localhost:8080/api/mcp \
  -H "Content-Type: application/json" \
  -d "{\"operation\":1,\"resource\":\"post\",\"data\":{\"title\":\"新文章\",\"slug\":\"new\",\"content\":\"内容\",\"status\":\"published\"}}"
```

**获取所有已发布文章：**
```bash
curl -X POST http://localhost:8080/api/mcp \
  -H "Content-Type: application/json" \
  -d "{\"operation\":5,\"resource\":\"post\",\"params\":{\"published\":true}}"
```

**更新分类：**
```bash
curl -X POST http://localhost:8080/api/mcp \
  -H "Content-Type: application/json" \
  -d "{\"operation\":3,\"resource\":\"category\",\"id\":\"分类 UUID\",\"data\":{\"name\":\"新名称\"}}"
```

## Markdown 语法支持

文章内容使用 Markdown 格式，支持以下语法：

### 基本语法

```markdown
# 标题
## 二级标题
### 三级标题

**粗体** *斜体*

- 列表项 1
- 列表项 2

[链接文本](https://example.com)

![图片描述](image.jpg)
```

### 代码块

````markdown
```rust
fn main() {
    println!("Hello, World!");
}
```
````

### 扩展语法

**表格：**
```markdown
| 列 1 | 列 2 | 列 3 |
|------|------|------|
| 值 1 | 值 2 | 值 3 |
```

**任务列表：**
```markdown
- [x] 已完成任务
- [ ] 未完成任务
```

**删除线：**
```markdown
~~删除的内容~~
```

## 多级分类示例

创建顶级分类：
```bash
curl -X POST http://localhost:8080/api/categories \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"编程\",\"slug\":\"programming\"}"
```

假设返回的 ID 是 `uuid-1`，创建子分类：
```bash
curl -X POST http://localhost:8080/api/categories \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"Rust\",\"slug\":\"rust\",\"parent_id\":\"uuid-1\"}"
```

获取分类树：
```bash
curl http://localhost:8080/api/categories/tree
```

## API 完整列表

### 文章相关
- `GET /api/posts` - 所有文章
- `GET /api/posts/published` - 已发布文章
- `GET /api/posts/{id}` - 指定文章
- `POST /api/posts` - 创建文章
- `PUT /api/posts/{id}` - 更新文章
- `DELETE /api/posts/{id}` - 删除文章

### 页面相关
- `GET /api/pages` - 所有页面
- `GET /api/pages/published` - 已发布页面
- `GET /api/pages/{id}` - 指定页面
- `POST /api/pages` - 创建页面
- `PUT /api/pages/{id}` - 更新页面
- `DELETE /api/pages/{id}` - 删除页面

### 分类相关
- `GET /api/categories` - 所有分类
- `GET /api/categories/tree` - 分类树
- `GET /api/categories/{id}` - 指定分类
- `POST /api/categories` - 创建分类
- `PUT /api/categories/{id}` - 更新分类
- `DELETE /api/categories/{id}` - 删除分类

### MCP 统一接口
- `POST /api/mcp` - AI Agent 操作接口

## 前端页面

访问以下 URL 查看博客前端：

- **首页**: http://localhost:8080/
- **文章列表**: http://localhost:8080/posts
- **分类**: http://localhost:8080/categories
- **文章详情**: http://localhost:8080/posts/{文章 ID}
- **静态页面**: http://localhost:8080/pages/{页面 slug}

## 常见问题

### 1. 数据库连接失败

确保：
- PostgreSQL 服务正在运行
- `.env` 文件中的 `DATABASE_URL` 正确
- 数据库 `blog_db` 已创建

### 2. 端口被占用

修改 `src/main.rs` 中的端口号：
```rust
.bind("0.0.0.0:8080")?  // 改为其他端口，如 3000
```

### 3. 模板文件找不到

确保在项目的根目录运行 `cargo run`，模板文件路径是相对于工作目录的。

## 开发技巧

### 热重载开发

使用 `cargo-watch` 实现自动重新编译：

```bash
cargo install cargo-watch
cargo watch -x run
```

### 日志级别

设置环境变量调整日志级别：

```bash
# Windows PowerShell
$env:RUST_LOG="debug"
cargo run

# Linux/Mac
export RUST_LOG=debug
cargo run
```

可选级别：error, warn, info, debug, trace

## 备份和恢复

### 数据库备份

```bash
pg_dump -U postgres blog_db > backup.sql
```

### 数据库恢复

```bash
psql -U postgres blog_db < backup.sql
```

## 下一步

现在你已经掌握了博客系统的基本使用方法，可以：

1. 创建更多文章和页面
2. 建立完整的分类体系
3. 自定义模板样式
4. 添加更多功能（标签、评论等）
5. 部署到生产环境

祝你使用愉快！🎉
