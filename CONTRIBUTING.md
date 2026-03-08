# 贡献指南

感谢你对本项目的关注！本文档将帮助你了解如何参与项目开发。

## 📖 目录结构

在开始之前，请确保你已了解项目结构：

```
blog/
├── src/              # 源代码
│   ├── db/          # 数据库层
│   ├── handlers/    # HTTP 处理器
│   ├── mcp/         # MCP 协议
│   ├── models/      # 数据模型
│   ├── templates/   # 前端模板
│   └── utils/       # 工具函数
├── migrations/       # 数据库迁移
└── docs/            # 文档（如果有的话）
```

## 🛠️ 开发环境设置

### 1. 安装必要工具

- Rust 1.70+ 
- PostgreSQL 12+
- Git

### 2. 克隆项目

```bash
git clone <repository-url>
cd blog
```

### 3. 配置数据库

```bash
# 创建数据库
psql -U postgres
CREATE DATABASE blog_db;
\q

# 复制环境变量
cp .env.example .env

# 编辑 .env 文件
# DATABASE_URL=postgres://username:password@localhost:5432/blog_db
```

### 4. 编译项目

```bash
cargo build
```

### 5. 运行测试

```bash
cargo test
```

## 🔀 贡献流程

### 1. Fork 项目

点击 GitHub 页面上的 "Fork" 按钮创建项目副本

### 2. 创建分支

```bash
git checkout -b feature/your-feature-name
```

分支命名规范：
- `feature/xxx` - 新功能
- `bugfix/xxx` - Bug 修复
- `docs/xxx` - 文档更新
- `refactor/xxx` - 代码重构
- `test/xxx` - 测试相关

### 3. 进行修改

请遵循以下编码规范：

#### 代码风格

```rust
// 使用有意义的变量名
let user_name = String::from("John");

// 添加必要的注释
/// 创建新用户
/// 
/// # Arguments
/// * `name` - 用户名称
pub fn create_user(name: &str) -> Result<User, Error> {
    // ...
}

// 错误处理使用 Result 类型
pub fn get_post(id: Uuid) -> Result<Post, AppError> {
    match repo.find_by_id(id).await? {
        Some(post) => Ok(post),
        None => Err(AppError::NotFound),
    }
}
```

#### 提交信息规范

```
feat: 添加文章搜索功能
fix: 修复分类树显示错误
docs: 更新 API 文档
refactor: 重构数据库连接池
test: 添加用户认证测试
```

格式：`type: subject`

type 包括：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建工具或依赖更新

### 4. 测试你的修改

```bash
# 运行所有测试
cargo test

# 检查代码格式
cargo fmt -- --check

# 运行 clippy 检查
cargo clippy -- -D warnings
```

### 5. 提交更改

```bash
git add .
git commit -m "feat: 添加新功能"
```

### 6. 推送到远程

```bash
git push origin feature/your-feature-name
```

### 7. 创建 Pull Request

在 GitHub 上创建 PR，描述你的修改：
- 修改的目的
- 实现的功能
- 测试情况
- 相关 Issue

## 📝 开发建议

### 添加新功能

1. **在正确的模块中添加代码**
   - 数据模型 → `src/models/`
   - 数据库操作 → `src/db/`
   - HTTP 处理器 → `src/handlers/`
   - 模板文件 → `src/templates/`

2. **更新相关文件**
   - 在 `mod.rs` 中导出新模块
   - 在 `main.rs` 中注册路由
   - 更新 API 文档

3. **编写测试**
   - 单元测试放在对应模块的 `tests` 子模块
   - 集成测试可以放在 `tests/` 目录

### 修复 Bug

1. **重现问题**
   - 确认 Bug 的存在
   - 记录重现步骤

2. **定位问题**
   - 使用日志调试
   - 添加断点（如需要）

3. **修复并测试**
   - 最小化修改
   - 确保不引入新问题

### 优化代码

1. **性能优化**
   - 使用基准测试证明改进
   - 不要过度优化

2. **代码清理**
   - 删除未使用的代码
   - 改进变量命名
   - 添加缺失的注释

## 🐛 报告 Bug

### Bug 报告模板

```markdown
**Bug 描述**
清晰简洁地描述这个 Bug

**重现步骤**
1. 第一步...
2. 第二步...
3. 看到错误...

**期望行为**
原本应该发生什么

**截图**
如果可以，添加截图帮助说明

**环境信息**
- OS: Windows 11
- Rust: 1.70
- PostgreSQL: 15
- 版本：v0.1.0

**其他信息**
任何你认为有帮助的信息
```

## 💡 功能建议

### 功能建议模板

```markdown
**功能描述**
清晰简洁地描述你想要的功能

**问题关联**
这个功能解决了什么问题？

**实现建议**
你有实现想法吗？如何实现？

**替代方案**
有没有其他解决方案？

**其他信息**
任何补充说明
```

## 📚 文档贡献

如果你发现文档有问题或有改进建议：

1. 直接提交 PR 修改文档
2. 在文档开头添加更新说明
3. 确保示例代码是最新的

## 🔍 Code Review 要点

审查代码时关注：

- ✅ 功能是否正确实现
- ✅ 代码是否符合规范
- ✅ 是否有合适的测试
- ✅ 性能影响
- ✅ 安全性考虑
- ✅ 文档是否完整

## 🎯 当前需求

我们特别欢迎以下类型的贡献：

- [ ] 单元测试和集成测试
- [ ] 性能优化
- [ ] 文档完善
- [ ] Bug 修复
- [ ] 新功能建议
- [ ] UI/UX 改进

## 📞 联系方式

如有问题，可以通过以下方式联系：

- 项目 Issues: GitHub Issues
- 邮件：[待添加]

## 🙏 致谢

感谢所有为这个项目做出贡献的人！

---

最后更新：2026-03-07
