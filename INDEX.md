# Rust 博客系统 - 项目索引

欢迎使用 Rust 博客系统！本文档将帮助你快速找到所需信息。

## 📚 文档导航

### 🚀 新手入门

1. **[README.md](README.md)** - 从这里开始！
   - 项目介绍和功能特性
   - 技术栈说明
   - 快速开始指南
   - API 接口概览

2. **[USAGE.md](USAGE.md)** - 详细使用教程
   - 环境配置步骤
   - 数据库设置
   - 完整的使用示例
   - Markdown 语法支持
   - 常见问题解答

3. **[CHEATSHEET.md](CHEATSHEET.md)** - 快速参考手册
   - 常用命令速查
   - API 端点一览
   - cURL 命令示例
   - MCP 协议格式
   - 故障排查技巧

### 📖 深入学习

4. **[PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)** - 项目完整总结
   - 功能详细说明
   - 技术架构解析
   - 数据库设计
   - 代码结构分析

5. **[CONTRIBUTING.md](CONTRIBUTING.md)** - 贡献指南
   - 开发环境设置
   - 代码规范
   - 提交流程
   - Bug 报告模板

6. **[CHANGELOG.md](CHANGELOG.md)** - 更新日志
   - 版本历史
   - 更新记录
   - 升级指南

7. **[CHECKLIST.md](CHECKLIST.md)** - 完成检查清单
   - 功能验证清单
   - 项目文件列表
   - 部署准备检查

### 🛠️ 工具资源

8. **[blog_api.postman_collection.json](blog_api.postman_collection.json)** - Postman API 测试集合
   - 导入 Postman 即可测试所有接口

9. **[test.sh](test.sh)** - 测试脚本
   - API 测试命令
   - 使用示例

10. **[start.bat](start.bat) / [start.sh](start.sh)** - 快速启动脚本
    - Windows: start.bat
    - Linux/Mac: start.sh

11. **[docker-compose.yml](docker-compose.yml)** - Docker 编排配置
    - 一键部署到 Docker

12. **[Dockerfile](Dockerfile)** - Docker 镜像构建文件
    - 多阶段构建优化

### ⚙️ 配置文件

13. **[Cargo.toml](Cargo.toml)** - 项目依赖配置
    - Rust 依赖管理
    - 编译配置

14. **[askama.toml](askama.toml)** - 模板引擎配置

15. **[.env.example](.env.example)** - 环境变量示例
    - 数据库连接配置

16. **[init_db.sql](init_db.sql)** - 数据库初始化脚本

17. **[.gitignore](.gitignore)** - Git 忽略文件配置

## 🎯 快速查找

### 我想...

#### 安装和运行项目
→ 查看 [README.md](README.md) 的"快速开始"部分
→ 或使用 [start.bat](start.bat) (Windows) / [start.sh](start.sh) (Linux/Mac)

#### 了解如何使用 API
→ 查看 [USAGE.md](USAGE.md) 的"管理博客内容"部分
→ 参考 [CHEATSHEET.md](CHEATSHEET.md) 的"API 端点一览"
→ 导入 [blog_api.postman_collection.json](blog_api.postman_collection.json) 到 Postman

#### 学习 MCP 协议
→ 查看 [USAGE.md](USAGE.md) 的"MCP 接口"部分
→ 参考 [CHEATSHEET.md](CHEATSHEET.md) 的"MCP 协议格式"

#### 自定义前端模板
→ 查看 `src/templates/` 目录中的模板文件
→ 参考 Tera 模板引擎文档

#### 添加新功能
→ 查看 [CONTRIBUTING.md](CONTRIBUTING.md)
→ 参考 [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) 了解架构

#### 排查问题
→ 查看 [USAGE.md](USAGE.md) 的"常见问题"部分
→ 参考 [CHEATSHEET.md](CHEATSHEET.md) 的"故障排查"

#### 了解数据库设计
→ 查看 `migrations/001_init.sql`
→ 参考 [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) 的"数据库设计"部分

#### 部署到生产环境
→ 查看 [USAGE.md](USAGE.md) 的部署相关章节
→ 使用 [docker-compose.yml](docker-compose.yml) 或 [Dockerfile](Dockerfile)

## 📂 源代码结构

```
src/
├── main.rs                 # 应用入口
├── db/                     # 数据库层
│   ├── mod.rs             # 数据库连接和迁移
│   ├── category_repository.rs
│   ├── post_repository.rs
│   └── page_repository.rs
├── handlers/               # HTTP 处理器
│   ├── mod.rs
│   ├── category_handler.rs
│   ├── post_handler.rs
│   ├── page_handler.rs
│   └── view_handler.rs     # 前端页面渲染
├── mcp/                    # MCP 协议实现
│   ├── mod.rs
│   ├── models.rs
│   └── handler.rs
├── models/                 # 数据模型
│   ├── mod.rs
│   ├── category.rs
│   ├── post.rs
│   └── page.rs
├── templates/              # Tera 模板
│   ├── index.html.tera
│   ├── posts.html.tera
│   ├── post.html.tera
│   ├── categories.html.tera
│   └── page.html.tera
└── utils/                  # 工具函数
    ├── mod.rs
    └── markdown.rs         # Markdown 转 HTML
```

## 🔑 核心概念

### RESTful API
标准的 HTTP 接口用于管理博客内容：
- 文章：`/api/posts`
- 页面：`/api/pages`
- 分类：`/api/categories`

### MCP (Model Context Protocol)
统一的 AI 操作接口：
- 操作类型：Create(1), Read(2), Update(3), Delete(4), List(5)
- 资源类型：post, page, category
- 端点：`POST /api/mcp`

### Markdown 渲染
文章内容使用 Markdown 格式存储，前端展示时转换为 HTML。

### 多级分类
支持无限层级的分类结构，通过 parent_id 建立父子关系。

## 📊 项目统计

- **源代码文件**: 20+
- **文档文件**: 10+
- **API 端点**: 25+
- **模板文件**: 5
- **数据库表**: 3

## 🆘 获取帮助

1. **查看文档**
   - 大部分问题都能在文档中找到答案
   - 从 README.md 开始

2. **检查示例**
   - test.sh 包含 API 测试示例
   - Postman 集合包含所有接口的用法

3. **查看源码注释**
   - 关键代码都有注释说明

4. **提交 Issue**
   - 在 GitHub 提交问题
   - 使用 CONTRIBUTING.md 中的模板

## 🎯 下一步

- ✅ **已完成**: 所有核心功能已实现
- 📋 **使用中**: 按照 USAGE.md 开始使用
- 🔧 **开发中**: 如需添加新功能，参考 CONTRIBUTING.md
- 📝 **学习中**: 阅读 PROJECT_SUMMARY.md 深入了解架构

## 📞 重要链接

- **项目主页**: 查看 README.md
- **使用教程**: USAGE.md
- **快速参考**: CHEATSHEET.md
- **API 文档**: 见 README.md 的"API 接口"部分
- **Postman 集合**: blog_api.postman_collection.json

---

## 📝 文档版本

- 项目版本：v0.1.0
- 文档最后更新：2026-03-07
- 状态：✅ 完成

---

💡 **提示**: 将此文件保存为 INDEX.md 或 START_HERE.md，作为项目的入口文档。

祝你使用愉快！🎉
