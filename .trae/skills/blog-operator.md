# Blog Operator Skill

## 技能名称

blog-operator

## 技能描述

用于运营和管理 Rust 博客系统的 AI Agent 技能。通过 MCP (Model Context Protocol) 协议与博客后端交互，支持文章、页面、分类的完整 CRUD 操作，以及博客内容运营工作流。

## 适用场景

- 博客内容创作与发布
- 文章管理与编辑
- 分类体系维护
- 静态页面管理
- 博客运营自动化

## 前置条件

1. 博客服务已启动并运行在 `http://localhost:8080`
2. 数据库已配置并连接
3. 具有 API 访问权限（默认无需认证，或配置 API Key）

## 快速开始

### 启动博客服务

```bash
# Windows
start.bat

# Linux/Mac
./start.sh

# 或手动启动
cargo run
```

### 验证服务状态

```bash
curl http://localhost:8080
```

## MCP 协议详解

### 端点

```
POST /api/mcp
Content-Type: application/json
```

### 操作类型 (Operation)

| 值 | 名称 | 说明 |
|----|------|------|
| 1 | Create | 创建资源 |
| 2 | Read | 读取资源 |
| 3 | Update | 更新资源 |
| 4 | Delete | 删除资源 |
| 5 | List | 列表查询 |

### 资源类型 (Resource)

- `post` - 文章
- `page` - 页面
- `category` - 分类
- `upload` - 文件上传

### 请求格式

```json
{
  "operation": 1,
  "resource": "post",
  "data": { ... },
  "id": "uuid-string",
  "params": { ... }
}
```

### 响应格式

```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "message": "操作成功"
}
```

## 数据模型

### 文章 (Post)

```rust
{
  "id": "string (uuid)",
  "title": "string",
  "slug": "string (URL友好标识)",
  "content": "string (Markdown格式)",
  "excerpt": "string? (摘要)",
  "category_id": "string? (分类UUID)",
  "status": "draft | published",
  "published_at": "datetime?",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### 页面 (Page)

```rust
{
  "id": "string (uuid)",
  "title": "string",
  "slug": "string",
  "content": "string (Markdown格式)",
  "is_published": "boolean",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### 分类 (Category)

```rust
{
  "id": "string (uuid)",
  "name": "string",
  "slug": "string",
  "description": "string?",
  "parent_id": "string? (父分类UUID)",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### 上传文件 (UploadFile)

```rust
{
  "id": "string",
  "original_name": "string (原始文件名)",
  "file_name": "string (存储文件名)",
  "file_path": "string (存储路径)",
  "file_size": "number (字节)",
  "mime_type": "string",
  "file_url": "string (访问URL)",
  "storage_type": "fs | s3",
  "created_at": "datetime"
}
```

## 操作示例

### 文章操作

#### 创建文章

```json
{
  "operation": 1,
  "resource": "post",
  "data": {
    "title": "文章标题",
    "slug": "article-slug",
    "content": "# 标题\n\n正文内容...",
    "excerpt": "文章摘要",
    "category_id": "分类UUID",
    "status": "published"
  }
}
```

#### 获取文章列表

```json
{
  "operation": 5,
  "resource": "post"
}
```

#### 获取已发布文章

```json
{
  "operation": 5,
  "resource": "post",
  "params": {
    "published": true
  }
}
```

#### 读取单篇文章

```json
{
  "operation": 2,
  "resource": "post",
  "id": "文章UUID"
}
```

#### 更新文章

```json
{
  "operation": 3,
  "resource": "post",
  "id": "文章UUID",
  "data": {
    "title": "新标题",
    "content": "新内容"
  }
}
```

#### 删除文章

```json
{
  "operation": 4,
  "resource": "post",
  "id": "文章UUID"
}
```

### 页面操作

#### 创建页面

```json
{
  "operation": 1,
  "resource": "page",
  "data": {
    "title": "关于我们",
    "slug": "about",
    "content": "# 关于我们\n\n公司介绍...",
    "is_published": true
  }
}
```

#### 获取页面列表

```json
{
  "operation": 5,
  "resource": "page"
}
```

#### 更新页面

```json
{
  "operation": 3,
  "resource": "page",
  "id": "页面UUID",
  "data": {
    "content": "更新后的内容"
  }
}
```

### 分类操作

#### 创建顶级分类

```json
{
  "operation": 1,
  "resource": "category",
  "data": {
    "name": "技术",
    "slug": "tech",
    "description": "技术相关文章"
  }
}
```

#### 创建子分类

```json
{
  "operation": 1,
  "resource": "category",
  "data": {
    "name": "Rust",
    "slug": "rust",
    "description": "Rust编程语言",
    "parent_id": "父分类UUID"
  }
}
```

#### 获取分类树

```json
{
  "operation": 5,
  "resource": "category",
  "params": {
    "tree": true
  }
}
```

### 文件上传操作

#### 获取文件列表 (MCP)

```json
{
  "operation": 5,
  "resource": "upload"
}
```

#### 读取文件信息 (MCP)

```json
{
  "operation": 2,
  "resource": "upload",
  "id": "文件ID"
}
```

#### 删除文件 (MCP)

```json
{
  "operation": 4,
  "resource": "upload",
  "id": "文件ID"
}
```

#### 上传文件 (REST API)

```bash
curl -X POST http://localhost:8080/api/admin/uploads \
  -H "X-API-Key: your-api-key" \
  -F "file=@/path/to/image.jpg"
```

#### 获取存储配置

```bash
curl http://localhost:8080/api/admin/storage/info \
  -H "X-API-Key: your-api-key"
```

## 运营工作流

### 1. 创建文章完整流程

```
1. 检查/创建分类 → 获取分类列表
2. 创建文章 → 指定分类和状态
3. 验证文章 → 读取确认
4. 发布文章 → 更新状态为 published
```

### 2. 内容审核流程

```
1. 获取草稿文章列表
2. 读取文章内容
3. 更新/发布或删除
```

### 3. 分类整理流程

```
1. 获取分类树
2. 分析分类结构
3. 创建/调整分类
4. 迁移文章到新分类
```

## 工具函数

### 生成 slug

将标题转换为 URL 友好的 slug：

```python
def generate_slug(title: str) -> str:
    """将标题转换为 slug"""
    import re
    slug = title.lower()
    slug = re.sub(r'[^\w\s-]', '', slug)
    slug = re.sub(r'[-\s]+', '-', slug)
    return slug.strip('-')
```

### 构建 MCP 请求

```python
def build_mcp_request(
    operation: int,
    resource: str,
    data: dict = None,
    id: str = None,
    params: dict = None
) -> dict:
    """构建 MCP 请求体"""
    request = {
        "operation": operation,
        "resource": resource
    }
    if data:
        request["data"] = data
    if id:
        request["id"] = id
    if params:
        request["params"] = params
    return request
```

## 常用 cURL 命令

### 测试 MCP 接口

```bash
# 创建文章
curl -X POST http://localhost:8080/api/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "operation": 1,
    "resource": "post",
    "data": {
      "title": "测试文章",
      "slug": "test-post",
      "content": "# 测试\n\n这是一篇测试文章",
      "status": "published"
    }
  }'

# 获取文章列表
curl -X POST http://localhost:8080/api/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "operation": 5,
    "resource": "post"
  }'

# 创建分类
curl -X POST http://localhost:8080/api/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "operation": 1,
    "resource": "category",
    "data": {
      "name": "技术",
      "slug": "tech"
    }
  }'
```

## 前端页面

博客前端页面可直接访问：

- `http://localhost:8080/` - 首页
- `http://localhost:8080/posts` - 文章列表
- `http://localhost:8080/posts/{id}` - 文章详情
- `http://localhost:8080/categories` - 分类列表
- `http://localhost:8080/pages/{slug}` - 静态页面

## 最佳实践

### 1. 文章发布

- 先创建为 `draft` 状态
- 审核后再更新为 `published`
- 使用有意义的 slug，便于 SEO

### 2. 分类管理

- 保持分类层级不超过 3 层
- 分类名称简洁明了
- 定期整理无文章分类

### 3. 内容格式

- 使用标准 Markdown 语法
- 添加适当的标题层级
- 为长文章添加摘要

### 4. 错误处理

- 检查 `success` 字段
- 处理 `error` 错误信息
- 验证返回数据完整性

## 故障排查

### 服务无法连接

```bash
# 检查服务状态
curl http://localhost:8080

# 查看日志
cargo run
```

### 数据库错误

```bash
# 检查数据库连接
psql -U postgres -d blog_db -c "SELECT 1"

# 查看表结构
psql -U postgres -d blog_db -c "\dt"
```

### MCP 请求失败

- 验证 JSON 格式
- 检查 operation 和 resource 值
- 确认必需字段存在

## 相关文件

- [项目说明](../../README.md)
- [使用指南](../../USAGE.md)
- [快速参考](../../CHEATSHEET.md)
- [API 集合](../../blog_api.postman_collection.json)

## 存储配置

### 本地文件系统 (默认)

```bash
# .env
STORAGE_TYPE=fs
UPLOAD_DIR=./uploads
BASE_URL=http://localhost:8080
```

### 阿里云 OSS

```bash
# .env
STORAGE_TYPE=s3
S3_BUCKET=your-bucket-name
S3_ENDPOINT=https://your-bucket.oss-cn-region.aliyuncs.com
S3_ACCESS_KEY=your-access-key
S3_SECRET_KEY=your-secret-key
S3_REGION=oss-cn-region
CDN_URL=https://your-cdn-domain.com
```

### MinIO

```bash
# .env
STORAGE_TYPE=s3
S3_BUCKET=your-bucket
S3_ENDPOINT=http://localhost:9000
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin
S3_REGION=us-east-1
```

### 支持的文件类型

- 图片: JPEG, PNG, GIF, WebP
- 文档: PDF, TXT, Markdown
- 最大文件大小: 10MB

## 技术栈

- **后端**: Actix Web + Rust
- **数据库**: PostgreSQL + SQLx
- **模板**: Askama
- **存储**: OpenDAL (支持 FS/S3/阿里云OSS/MinIO)
- **MCP 协议**: 自定义 JSON 协议
