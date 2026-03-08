# 快速参考手册

## 🚀 快速启动

### Windows
```bash
start.bat
```

### Linux/Mac
```bash
chmod +x start.sh
./start.sh
```

### 手动启动
```bash
# 1. 配置数据库
cp .env.example .env
# 编辑 .env 设置 DATABASE_URL

# 2. 运行
cargo run
```

## 📡 API 端点一览

### 文章 (Posts)
```
GET    /api/posts              # 所有文章
GET    /api/posts/published    # 已发布文章
GET    /api/posts/{id}         # 获取文章
GET    /api/posts/slug/{slug}  # 通过 slug 获取
POST   /api/posts              # 创建文章
PUT    /api/posts/{id}         # 更新文章
DELETE /api/posts/{id}         # 删除文章
```

### 页面 (Pages)
```
GET    /api/pages              # 所有页面
GET    /api/pages/published    # 已发布页面
GET    /api/pages/{id}         # 获取页面
GET    /api/pages/slug/{slug}  # 通过 slug 获取
POST   /api/pages              # 创建页面
PUT    /api/pages/{id}         # 更新页面
DELETE /api/pages/{id}         # 删除页面
```

### 分类 (Categories)
```
GET    /api/categories          # 所有分类
GET    /api/categories/tree     # 分类树
GET    /api/categories/{id}     # 获取分类
POST   /api/categories          # 创建分类
PUT    /api/categories/{id}     # 更新分类
DELETE /api/categories/{id}     # 删除分类
```

### MCP 统一接口
```
POST   /api/mcp                # AI Agent 操作接口
```

## 🌐 前端页面

```
/                        # 首页
/posts                  # 文章列表
/posts/{id}             # 文章详情
/categories             # 分类
/pages/{slug}           # 静态页面
```

## 🔧 常用 cURL 命令

### 创建文章
```bash
curl -X POST http://localhost:8080/api/posts \
  -H "Content-Type: application/json" \
  -d '{"title":"标题","slug":"my-post","content":"# 内容","status":"published"}'
```

### 创建分类
```bash
curl -X POST http://localhost:8080/api/categories \
  -H "Content-Type: application/json" \
  -d '{"name":"技术","slug":"tech"}'
```

### 创建页面
```bash
curl -X POST http://localhost:8080/api/pages \
  -H "Content-Type: application/json" \
  -d '{"title":"关于","slug":"about","content":"内容","is_published":true}'
```

### MCP 创建文章
```bash
curl -X POST http://localhost:8080/api/mcp \
  -H "Content-Type: application/json" \
  -d '{"operation":1,"resource":"post","data":{"title":"MCP 文章","content":"内容","status":"published"}}'
```

### MCP 获取文章列表
```bash
curl -X POST http://localhost:8080/api/mcp \
  -H "Content-Type: application/json" \
  -d '{"operation":5,"resource":"post","params":{"published":true}}'
```

## 📊 MCP 协议格式

### 请求格式
```json
{
  "operation": 1-5,        // 1=Create, 2=Read, 3=Update, 4=Delete, 5=List
  "resource": "post|page|category",
  "data": {...},           // 可选，操作数据
  "id": "uuid",            // 可选，资源 ID
  "params": {...}          // 可选，额外参数
}
```

### 响应格式
```json
{
  "success": true/false,
  "data": {...},           // 返回数据
  "error": "错误信息",      // 错误时填写
  "message": "提示信息"     // 成功时填写
}
```

## 🗄️ 数据库操作

### 创建数据库
```sql
CREATE DATABASE blog_db;
```

### 备份数据库
```bash
pg_dump -U postgres blog_db > backup.sql
```

### 恢复数据库
```bash
psql -U postgres blog_db < backup.sql
```

## 🔍 故障排查

### 数据库连接失败
```bash
# 检查 PostgreSQL 服务状态
# Windows: 服务管理器查看 PostgreSQL 服务
# Linux: systemctl status postgresql

# 测试连接
psql -U postgres -h localhost -p 5432
```

### 端口被占用
```bash
# 修改 src/main.rs 中的端口
.bind("0.0.0.0:3000")?  # 改为 3000
```

### 编译错误
```bash
# 清理并重新编译
cargo clean
cargo build
```

### 模板文件找不到
确保在项目根目录运行：
```bash
cd C:\Users\micross\RustroverProjects\blog
cargo run
```

## 📝 Markdown 语法速查

### 基本语法
```markdown
# 标题
**粗体** *斜体*
- 列表项
[链接](url)
![图片](url)
```

### 代码
````markdown
行内代码：`code`

代码块：
```rust
fn main() {
    println!("Hello");
}
```
````

### 表格
```markdown
| 列 1 | 列 2 |
|------|------|
| 值 1 | 值 2 |
```

### 任务列表
```markdown
- [x] 已完成
- [ ] 未完成
```

### 引用
```markdown
> 引用内容
```

## 🎯 开发技巧

### 热重载
```bash
cargo install cargo-watch
cargo watch -x run
```

### 调试日志
```bash
# Windows PowerShell
$env:RUST_LOG="debug"
cargo run

# Linux/Mac
export RUST_LOG=debug
cargo run
```

### 生产环境编译
```bash
cargo build --release
```

## 🐳 Docker 部署

### 使用 docker-compose
```bash
docker-compose up -d
```

### 查看日志
```bash
docker-compose logs -f blog
```

### 停止服务
```bash
docker-compose down
```

## 📞 有用的链接

- 本地访问：http://localhost:8080
- API 测试：使用 Postman 导入 `blog_api.postman_collection.json`
- 文档位置：README.md, USAGE.md, PROJECT_SUMMARY.md

---

💡 **提示**: 将此文件保存为 CHEATSHEET.md 以便快速查阅
