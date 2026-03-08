# 更新日志 / Change Log

本文档记录项目的所有重要更新。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### 新增
- 完整的博客系统实现
- 文章管理功能（CRUD）
- 页面管理功能（CRUD）
- 多级分类系统
- Markdown 内容渲染
- RESTful API 接口
- MCP (Model Context Protocol) AI Agent 接口
- Tera 模板引擎前端展示
- PostgreSQL 数据库支持
- Docker 部署配置

### 技术栈
- Actix Web 4.x - Web 框架
- SQLx 0.7 - 数据库 ORM
- Tera 1.x - 模板引擎
- pulldown-cmark 0.9 - Markdown 渲染
- Serde - 序列化
- Tokio - 异步运行时

---

## [0.1.0] - 2026-03-07

### 首次发布

#### 核心功能
- ✅ 文章管理系统
  - 创建、编辑、删除文章
  - 草稿和发布状态
  - Markdown 内容存储和渲染
  - 通过 ID 或 slug 访问
  
- ✅ 页面管理系统
  - 静态页面创建和管理
  - Markdown 渲染
  - 发布控制
  
- ✅ 多级分类系统
  - 无限层级分类
  - 父子关系维护
  - 树形结构展示
  
- ✅ RESTful API
  - 完整的 CRUD 接口
  - 符合 REST 规范
  - JSON 响应格式
  
- ✅ MCP 协议
  - 统一的 AI 操作接口
  - 支持 Create/Read/Update/Delete/List 操作
  - AI Agent 友好设计
  
- ✅ 前端展示
  - 响应式设计
  - 美观的 UI
  - Markdown 样式优化

#### 技术特性
- ✅ PostgreSQL 数据库连接池
- ✅ 数据库迁移系统
- ✅ Repository 模式
- ✅ 错误处理机制
- ✅ 日志系统
- ✅ 环境变量配置

#### 文档和工具
- ✅ README.md - 项目说明
- ✅ USAGE.md - 使用指南
- ✅ PROJECT_SUMMARY.md - 项目总结
- ✅ CHEATSHEET.md - 快速参考
- ✅ CONTRIBUTING.md - 贡献指南
- ✅ CHECKLIST.md - 完成检查清单
- ✅ Postman API 测试集合
- ✅ Docker 部署配置
- ✅ 快速启动脚本（Windows/Linux/Mac）

#### 数据库
- ✅ posts 表 - 文章存储
- ✅ pages 表 - 页面存储
- ✅ categories 表 - 分类存储
- ✅ 索引优化
- ✅ 外键约束

---

## 版本说明

### 版本号规则

- **主版本号（Major）**：不兼容的 API 修改
- **次版本号（Minor）**：向下兼容的功能性新增
- **修订号（Patch）**：向下兼容的问题修正

### 更新类型说明

- **新增（Added）**：新功能
- **修改（Changed）**：现有功能的变更
- **弃用（Deprecated）**：即将移除的功能
- **移除（Removed）**：已移除的功能
- **修复（Fixed）**：Bug 修复
- **安全（Security）**：安全性修复

---

## 计划中的功能

### v0.2.0 (计划中)
- [ ] 用户认证系统
- [ ] 标签功能
- [ ] 评论系统
- [ ] RSS 订阅
- [ ] 搜索功能

### v0.3.0 (未来)
- [ ] 多语言支持
- [ ] 主题系统
- [ ] 图片上传
- [ ] 统计分析
- [ ] SEO 优化

### v1.0.0 (未来)
- [ ] 稳定的 API
- [ ] 完整的测试覆盖
- [ ] 性能优化
- [ ] 生产环境就绪

---

## 升级指南

### 从 v0.0.x 升级到 v0.1.0

这是首个正式版本，无需升级步骤。

**首次安装：**

1. 克隆项目
```bash
git clone <repository-url>
cd blog
```

2. 配置数据库
```bash
cp .env.example .env
# 编辑 .env 设置 DATABASE_URL
```

3. 运行
```bash
cargo run
```

---

## 已知问题

### v0.1.0

- 暂无已知问题

如发现问题，请在 GitHub Issues 报告。

---

## 贡献者

感谢所有为这个项目做出贡献的人！

（按贡献时间排序）

---

## 许可证

本项目采用 MIT 许可证。详见 LICENSE 文件。

---

**[未发布]**: 比较 HEAD...main
**[0.1.0]**: 初始发布

最后更新：2026-03-07
