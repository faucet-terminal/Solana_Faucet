# Solana Faucet

Solana Faucet 是一个用于在 Solana 测试网络上管理账户和执行基本转账功能的项目。该项目包括自动创建新账户、请求空投和执行转账功能，并在本地进行测试。
## 目录结构
```plaintext
Solana_Faucet/
├── Cargo.toml
├── src/
│ ├── main.rs
│ ├── lib.rs
│ ├── accounts.rs
│ ├── transactions.rs
└── tests/
└── transfer.rs
```
## 依赖项

本项目依赖以下库：

- `solana-client`：Solana 客户端库，用于与 Solana 节点进行交互。
- `solana-sdk`：Solana 软件开发工具包。
- `tokio`：异步运行时，用于异步操作。
- `shellexpand`：用于路径展开。

## 构建与运行
### 构建项目
- cargo build
### 运行项目
- cargo run
### 运行本地测试，并查看详细输出
- cargo test -- --nocapture

## 环境配置
### 安装 Rust
- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
### 配置 Solana CLI并添加 Solana CLI 到环境变量
- export PATH="/home/your-user/.local/share/solana/install/active_release/bin:$PATH"
### 配置 Solana 网络
- solana config set --url https://api.testnet.solana.com
- 我这边使用的test网络，还有dev网络，可以自行选用
