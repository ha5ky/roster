# Roster Service (名册服务)

🚀 一个基于 Rust 构建的高性能、异步后端服务，专为高效管理游戏名册和用户数据而设计。

## 📋 概览

Roster Service 提供了一套强大的 RESTful API 用于查询和管理名册数据。它利用 Rust 生态系统的优势，确保了类型安全、高并发能力以及极低的资源占用。

## ✨ 特性

- **RESTful API**: 基于 [Axum](https://github.com/tokio-rs/axum) 构建，这是一个快速且符合人体工程学的 Rust Web 框架。
- **数据库集成**: 使用 [SQLx](https://github.com/launchbadge/sqlx) 进行异步 MySQL 查询，支持编译时 SQL 语法检查。
- **动态筛选**: 支持复杂的查询参数，实现灵活的数据检索。
- **OpenAPI 文档**: 集成 [Utoipa](https://github.com/juhaku/utoipa)，自动生成 Swagger UI 接口文档。
- **跨平台构建**: 包含 `Makefile`，支持无缝构建和交叉编译（Linux/macOS/Windows）。

## 🛠 技术栈

- **语言**: Rust 2021 Edition
- **Web 框架**: Axum 0.7
- **数据库**: MySQL 8.0+
- **ORM/驱动**: SQLx (异步 MySQL 驱动)
- **序列化**: Serde & Serde JSON
- **配置管理**: Config-rs (基于 YAML)
- **日志**: Tracing & Tracing Subscriber

## ⚙️ 前置要求

- **Rust**: 最新稳定版 ([安装 Rust](https://www.rust-lang.org/tools/install))
- **MySQL**: 8.0 或兼容版本
- **Make**: 用于执行 Makefile 命令（可选但推荐）

## 🚀 快速开始

### 1. 克隆仓库

```bash
git clone <repository_url>
cd roster
```

### 2. 配置

确保项目根目录下存在 `config.yaml` 文件。你可以根据你的环境修改它：

```yaml
server:
  host: "127.0.0.1"
  port: your port

database:
  url: "mysql://root:your_password@localhost:{port}/your_database"
```

### 3. 数据库初始化

应用程序在启动时会尝试创建必要的表，但你也可以使用提供的导入脚本来初始化数据：

```bash
# 从 JSON 文件导入初始名册数据
cargo run --bin import_roster
```

### 4. 运行应用

使用 Makefile（推荐）：
```bash
make run
```

或者直接使用 Cargo：
```bash
cargo run
```

### 5. 访问 API

- **API 基础地址**: `http://localhost:3001`
- **Swagger UI 文档**: `http://localhost:3001/swagger-ui/`

## 📦 构建与部署

### 本地构建
```bash
make build
```
编译后的二进制文件位于 `target/release/roster`。

### 交叉编译（适用于 Linux 服务器）
如果你在 macOS 上开发，但需要部署到 Linux 服务器：

```bash
# 构建 Linux x86_64 版本
make build-linux-amd64
```
*注意：你可能需要通过 `rustup target add x86_64-unknown-linux-gnu` 安装目标架构，或使用 `cross` 工具。*

### 部署清单
要部署到服务器，请确保上传以下文件到**同一目录**：
1. 编译好的 **二进制文件** (`roster`)。
2. **配置文件** (`config.yaml`)。

```bash
# 服务器上的目录结构示例
/app
├── roster
└── config.yaml
```

在服务器上运行：
```bash
cd /app
./roster
```

## 📂 项目结构

```
roster/
├── src/
│   ├── bin/             # 独立脚本 (例如：数据导入工具)
│   ├── config.rs        # 配置逻辑
│   ├── db.rs            # 数据库连接与初始化
│   ├── error.rs         # 统一错误处理
│   ├── main.rs          # 应用入口与路由配置
│   ├── state.rs         # 共享应用状态
│   ├── roster/          # Roster 领域模块 (Handlers, Models, Repository)
│   └── users/           # User 领域模块
├── config.yaml          # 运行时配置文件
├── Cargo.toml           # 项目依赖
└── Makefile             # 构建自动化脚本
```

## 📝 许可证

本项目采用 MIT 许可证。
