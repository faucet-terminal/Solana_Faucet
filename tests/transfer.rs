use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use tokio;
use solana_faucet::accounts::{load_or_create_account, ensure_balance};
use solana_faucet::transactions::transfer;
use std::time::Duration;
use std::thread::sleep;

const MAX_RETRIES: usize = 10; // 增加重试次数
const RETRY_DELAY: Duration = Duration::from_secs(5); // 增加重试等待时间

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_transfer() {
    let rpc_url = "https://api.testnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let keypair_path = "~/.config/solana/id.json";
    let from_keypair = load_or_create_account(keypair_path).expect("加载/创建账户失败");

    let mut retries = 0;
    while retries < MAX_RETRIES {
        match ensure_balance(&client, &from_keypair, 1_000_000_000).await {
            Ok(_) => break,
            Err(err) => {
                println!("airdrop失败 (attempt {} of {}): {:?}", retries + 1, MAX_RETRIES, err);
                if retries >= MAX_RETRIES - 1 {
                    panic!("airdrop失败失败重试 {} 次", MAX_RETRIES);
                }
                sleep(RETRY_DELAY);
                retries += 1;
            }
        }
    }

    let to_pubkey = transfer(&client, &from_keypair, 1_000_000).await.expect("转账失败");

    assert!(!to_pubkey.is_empty());
}
