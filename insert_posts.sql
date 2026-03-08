-- 插入测试文章
INSERT INTO posts (title, slug, content, excerpt, category_id, status, published_at)
VALUES (
    'Rust 异步编程入门指南',
    'rust-async-guide',
    E'# Rust 异步编程入门指南\n\n## 什么是异步编程？\n\n异步编程是一种编程范式，允许程序在等待 I/O 操作完成时继续执行其他任务。\n\n## Rust 中的 async/await\n\nRust 使用 async/await 语法来支持异步编程。\n\n## 总结\n\n掌握异步编程是编写高性能 Rust 应用的关键。',
    '本文介绍 Rust 异步编程的基础知识',
    'a',
    'published',
    NOW()
);

INSERT INTO posts (title, slug, content, excerpt, category_id, status, published_at)
VALUES (
    'PostgreSQL 性能优化实践',
    'postgresql-performance',
    E'# PostgreSQL 性能优化实践\n\n## 索引优化\n\n合理的索引可以大幅提升查询性能。\n\n## 查询优化\n\n- 使用 EXPLAIN ANALYZE 分析查询计划\n- 避免 SELECT *\n- 使用连接池\n\n## 总结\n\n数据库优化是一个持续的过程。',
    '分享 PostgreSQL 数据库性能优化的实践经验',
    'a',
    'published',
    NOW()
);

INSERT INTO posts (title, slug, content, excerpt, category_id, status, published_at)
VALUES (
    'Web 开发安全最佳实践',
    'web-security-best-practices',
    E'# Web 开发安全最佳实践\n\n## 常见安全威胁\n\n1. SQL 注入\n2. XSS 攻击\n3. CSRF 攻击\n\n## 安全头部\n\n配置正确的安全头部可以防范多种攻击。\n\n## 总结\n\n安全是开发过程中的首要考虑。',
    '介绍 Web 开发中常见的安全威胁及防护措施',
    'a',
    'published',
    NOW()
);

INSERT INTO posts (title, slug, content, excerpt, category_id, status, published_at)
VALUES (
    'Docker 容器化部署指南',
    'docker-deployment-guide',
    E'# Docker 容器化部署指南\n\n## 什么是 Docker？\n\nDocker 是一个开源的容器化平台，可以让开发者打包应用及其依赖到一个可移植的容器中。\n\n## 基本命令\n\n```bash\ndocker build -t myapp .\ndocker run -p 8080:8080 myapp\n```\n\n## 总结\n\n容器化是现代应用部署的标准方式。',
    '详细介绍如何使用 Docker 进行应用容器化部署',
    'a',
    'published',
    NOW()
);

INSERT INTO posts (title, slug, content, excerpt, category_id, status, published_at)
VALUES (
    'Git 工作流最佳实践',
    'git-workflow-best-practices',
    E'# Git 工作流最佳实践\n\n## 分支策略\n\n- main: 生产分支\n- develop: 开发分支\n- feature/*: 功能分支\n\n## 提交规范\n\n使用语义化的提交信息。\n\n## 总结\n\n良好的 Git 工作流可以提高团队协作效率。',
    '分享团队开发中的 Git 分支管理和提交规范',
    'a',
    'published',
    NOW()
);
