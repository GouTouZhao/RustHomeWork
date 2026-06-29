# 环境设置
FROM rust:alpine AS builder

ARG SERVICE

# 工作目录设置
ENV SERVER_DIR=/forum
WORKDIR ${SERVER_DIR}

# 安装编译依赖
RUN apk add --no-cache musl-dev protobuf-dev

# 复制文件到工作区
COPY . .

# 编译特定服务
RUN cargo build --release --bin ${SERVICE}
RUN mkdir -p /output/bin && cp target/release/${SERVICE} /output/bin/app

# 第二阶段只要编译结果缩小容量
FROM alpine:latest

WORKDIR /app

COPY --from=builder /output/bin/app .

CMD ["./app"]
