# 博客系统测试脚本

## 1. 编译项目
cargo build --release

## 2. 运行项目
cargo run

## 3. 测试 API 接口

### 创建分类
curl -X POST http://localhost:8080/api/categories \
  -H "Content-Type: application/json" \
  -d '{
    "name": "技术文章",
    "slug": "tech",
    "description": "技术相关的文章"
  }'

### 创建文章
curl -X POST http://localhost:8080/api/posts \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Hello World",
    "slug": "hello-world",
    "content": "# Hello World\n\n这是我的第一篇文章！\n\n```rust\nfn main() {\n    println!(\"Hello, World!\");\n}\n```",
    "excerpt": "我的第一篇文章",
    "status": "published"
  }'

### 获取所有文章
curl http://localhost:8080/api/posts

### 获取已发布文章
curl http://localhost:8080/api/posts/published

### 通过 MCP 创建页面
curl -X POST http://localhost:8080/api/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "operation": 1,
    "resource": "page",
    "data": {
      "title": "关于我",
      "slug": "about",
      "content": "# 关于我\n\n欢迎来到我的博客！",
      "is_published": true
    }
  }'

### 通过 MCP 获取文章列表
curl -X POST http://localhost:8080/api/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "operation": 5,
    "resource": "post",
    "params": {
      "published": true
    }
  }'

## 4. 访问前端页面

- 首页：http://localhost:8080/
- 文章列表：http://localhost:8080/posts
- 分类：http://localhost:8080/categories
- 关于页面：http://localhost:8080/pages/about
