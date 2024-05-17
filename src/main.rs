mod accounts;
mod transactions;

use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use accounts::{load_or_create_account, ensure_balance};
use transactions::transfer;

#[tokio::main]
async fn main() {
    // Solana RPC 客户端配置
    let rpc_url = "https://api.testnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // 密钥对文件路径
    let keypair_path = "~/.config/solana/id.json";

    // 加载或创建账户
    let from_keypair = load_or_create_account(keypair_path).expect("Failed to load or create account");

    // 确保账户余额足够
    ensure_balance(&client, &from_keypair, 1_000_000_000).await.expect("Failed to ensure balance");

    // 执行转账
    let to_pubkey = transfer(&client, &from_keypair, 1_000_000).await.expect("Failed to transfer");

    println!("Transfer successful to {}", to_pubkey);
}
