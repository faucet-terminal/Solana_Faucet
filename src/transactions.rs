use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer}, // 确保导入 Signer trait
    system_instruction,
    transaction::Transaction,
};
use std::error::Error;

/// 执行从一个账户到另一个账户的转账
pub async fn transfer(client: &RpcClient, from_keypair: &Keypair, amount: u64) -> Result<String, Box<dyn Error>> {
    // 创建接收者账户
    let to_keypair = Keypair::new();
    let to_pubkey = to_keypair.pubkey(); // 使用 pubkey 方法获取接收者的公钥

    // 构建转账指令
    let ix = system_instruction::transfer(&from_keypair.pubkey(), &to_pubkey, amount);

    // 获取最新的区块哈希
    let recent_blockhash = client.get_latest_blockhash()?;
    
    // 创建并签名交易
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&from_keypair.pubkey()),
        &[from_keypair], // 这里去掉引用符号
        recent_blockhash,
    );

    // 发送交易并确认
    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Transaction successful with signature: {}", signature);

    // 获取接收者账户余额
    let to_balance = client.get_balance(&to_pubkey)?;
    println!("Recipient (B) balance: {}", to_balance);

    Ok(signature.to_string())
}
