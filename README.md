# Rust 博客系统

一个使用 Actix Web、PostgreSQL、SQLx 和 Askama 模板引擎构建的现代化博客系统。

## 功能特性

- ✅ **文章管理** - 支持 Markdown 格式，自动转换为 HTML
- ✅ **页面管理** - 静态页面（如关于页面）
- ✅ **多级分类** - 支持无限层级的分类结构
- ✅ **RESTful API** - 完整的管理接口
- ✅ **MCP 协议** - AI Agent 可直接运营管理
- ✅ **前端展示** - 简洁美观的响应式设计
- ✅ **Markdown 渲染** - 支持表格、任务列表、删除线等扩展语法

## 技术栈

- **后端框架**: Actix Web
- **数据库**: PostgreSQL + SQLx
- **模板引擎**: Askama
- **Markdown 渲染**: pulldown-cmark
- **序列化**: Serde + serde_json

## 快速开始

### 1. 环境要求

- Rust 1.70+
- PostgreSQL 12+

### 2. 安装依赖

```bash
cargo build
```

### 3. 配置数据库

创建 PostgreSQL 数据库：

```sql
CREATE DATABASE blog_db;
```

复制环境变量文件并修改：

```bash
cp .env.example .env
```

编辑 `.env` 文件，设置正确的数据库连接字符串：

```
DATABASE_URL=postgres://username:password@localhost:5432/blog_db
```

### 4. 运行应用

```bash
cargo run
```

服务器将在 `http://0.0.0.0:8080` 启动。

## 项目结构

```
blog/
├── src/
│   ├── db/                  # 数据库相关
│   │   ├── mod.rs          # 数据库连接和迁移
│   │   ├── category_repository.rs
│   │   ├── post_repository.rs
│   │   └── page_repository.rs
│   ├── handlers/            # HTTP 处理器
│   │   ├── mod.rs
│   │   ├── category_handler.rs
│   │   ├── post_handler.rs
│   │   ├── page_handler.rs
│   │   └── view_handler.rs  # 前端页面渲染
│   ├── mcp/                 # MCP 协议实现
│   │   ├── mod.rs
│   │   ├── models.rs       # MCP 数据模型
│   │   └── handler.rs      # MCP 请求处理
│   ├── models/              # 数据模型
│   │   ├── mod.rs
│   │   ├── category.rs
│   │   ├── post.rs
│   │   └── page.rs
│   ├── templates/           # Tera 模板
│   │   ├── index.html.tera
│   │   ├── posts.html.tera
│   │   ├── post.html.tera
│   │   ├── categories.html.tera
│   │   └── page.html.tera
│   ├── utils/               # 工具函数
│   │   ├── mod.rs
│   │   └── markdown.rs     # Markdown 转 HTML
│   └── main.rs             # 入口文件
├── migrations/              # 数据库迁移
│   └── 001_init.sql
├── Cargo.toml
└── .env.example
```

## API 接口

### 文章 API

- `GET /api/posts` - 获取所有文章
- `GET /api/posts/published` - 获取已发布文章
- `GET /api/posts/{id}` - 获取指定文章
- `GET /api/posts/slug/{slug}` - 通过 slug 获取文章
- `POST /api/posts` - 创建文章
- `PUT /api/posts/{id}` - 更新文章
- `DELETE /api/posts/{id}` - 删除文章
- `GET /api/posts/{id}/html` - 获取文章的 HTML 版本

### 页面 API

- `GET /api/pages` - 获取所有页面
- `GET /api/pages/published` - 获取已发布页面
- `GET /api/pages/{id}` - 获取指定页面
- `GET /api/pages/slug/{slug}` - 通过 slug 获取页面
- `POST /api/pages` - 创建页面
- `PUT /api/pages/{id}` - 更新页面
- `DELETE /api/pages/{id}` - 删除页面
- `GET /api/pages/{id}/html` - 获取页面的 HTML 版本

### 分类 API

- `GET /api/categories` - 获取所有分类
- `GET /api/categories/tree` - 获取分类树（多级结构）
- `GET /api/categories/{id}` - 获取指定分类
- `POST /api/categories` - 创建分类
- `PUT /api/categories/{id}` - 更新分类
- `DELETE /api/categories/{id}` - 删除分类

### MCP 接口

- `POST /api/mcp` - MCP 统一接口

MCP 请求格式：

```json
{
  "operation": 1,  // 1=Create, 2=Read, 3=Update, 4=Delete, 5=List
  "resource": "post",  // "post", "page", "category"
  "data": { ... },  // 可选，操作数据
  "id": "uuid-string",  // 可选，资源 ID
  "params": { ... }  // 可选，额外参数
}
```

MCP 响应格式：

```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "message": "操作成功"
}
```

## 前端页面

- `/` - 首页（显示最新文章）
- `/posts` - 文章列表
- `/posts/{id}` - 文章详情
- `/categories` - 分类列表
- `/pages/{slug}` - 静态页面

## 数据模型

### 文章 (Post)

```rust
{
  "id": "uuid",
  "title": "文章标题",
  "slug": "文章别名",
  "content": "Markdown 内容",
  "excerpt": "摘要",
  "category_id": "分类 UUID",
  "status": "draft|published",
  "published_at": "发布时间",
  "created_at": "创建时间",
  "updated_at": "更新时间"
}
```

### 页面 (Page)

```rust
{
  "id": "uuid",
  "title": "页面标题",
  "slug": "页面别名",
  "content": "Markdown 内容",
  "is_published": true,
  "created_at": "创建时间",
  "updated_at": "更新时间"
}
```

### 分类 (Category)

```rust
{
  "id": "uuid",
  "name": "分类名称",
  "slug": "分类别名",
  "description": "描述",
  "parent_id": "父分类 UUID",  // 支持多级分类
  "created_at": "创建时间",
  "updated_at": "更新时间"
}
```

## MCP 使用示例

AI Agent 可以通过 MCP 接口进行运营管理：

### 创建文章

```json
POST /api/mcp
{
  "operation": 1,
  "resource": "post",
  "data": {
    "title": "我的第一篇文章",
    "slug": "my-first-post",
    "content": "# Hello World\n\n这是我的文章内容...",
    "status": "published"
  }
}
```

### 更新分类

```json
POST /api/mcp
{
  "operation": 3,
  "resource": "category",
  "id": "category-uuid",
  "data": {
    "name": "新技术"
  }
}
```

### 获取文章列表

```json
POST /api/mcp
{
  "operation": 5,
  "resource": "post",
  "params": {
    "published": true
  }
}
```

## 开发说明

### Markdown 扩展语法

系统支持以下 Markdown 扩展：

- 表格 (Tables)
- 任务列表 (Task lists)
- 删除线 (Strikethrough)
- 自动链接 (Autolinks)

### 数据库迁移

系统会在首次启动时自动运行数据库迁移，创建所需的表结构。

### 环境变量

必需的环境变量：

- `DATABASE_URL` - PostgreSQL 连接字符串

可选的环境变量：

- `RUST_LOG` - 日志级别（默认：info）

## License

MIT License
