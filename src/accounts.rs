use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file, write_keypair_file},
};
use std::path::Path;
use std::error::Error;

/// 加载本地账户，如果没有则创建一个新账户
pub fn load_or_create_account(path: &str) -> Result<Keypair, Box<dyn Error>> {
    println!("本地账户地址: {}", path);
    let expanded_path = shellexpand::tilde(path).to_string();
    if Path::new(&expanded_path).exists() {
        // 从文件加载现有密钥对
        let keypair = read_keypair_file(&expanded_path)?;
        Ok(keypair)
    } else {
        // 创建新的密钥对并保存到文件
        let new_keypair = Keypair::new();
        write_keypair_file(&new_keypair, &expanded_path)?;
        println!("Created new account with public key: {}", new_keypair.pubkey());
        Ok(new_keypair)
    }
}

/// 确保账户有足够的余额，如果不足则请求空投
pub async fn ensure_balance(client: &RpcClient, keypair: &Keypair, min_balance: u64) -> Result<(), Box<dyn Error>> {
    let balance = client.get_balance(&keypair.pubkey())?;
    if balance < min_balance {
        client.request_airdrop(&keypair.pubkey(), min_balance - balance)?;
        let new_balance = client.get_balance(&keypair.pubkey())?;
        println!("New balance after airdrop: {}", new_balance);
    }
    Ok(())
}
