# 多阶段构建 - 编译阶段
FROM rust:1.93-alpine AS builder

WORKDIR /app

# 安装依赖
RUN apk add --no-cache bash curl npm libc-dev binaryen musl-dev clang lld openssl-libs-static libpq-dev

# 复制源代码
COPY . .

RUN npm install && \
    npm run build:css

# 编译项目
RUN cargo build --release

# # 运行阶段
FROM alpine

WORKDIR /app

# 安装运行时依赖
RUN apk add --no-cache tzdata ca-certificates

# 从编译阶段复制可执行文件
COPY --from=builder /app/target/release/blog .
COPY --from=builder /app/static ./static

ENV TZ=Asia/Shanghai

# 暴露端口
EXPOSE 8080

# 启动应用
CMD ["./blog"]
