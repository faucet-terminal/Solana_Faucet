use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use bs58;
use std::convert::TryInto;
use axum::{
    routing::post, Json, Router
};
use serde::{Deserialize, Serialize};

mod error;
use error::FaucetError;

///
/// DevNet: https://api.devnet.solana.com
/// TestNet: https://api.testnet.solana.com
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build router
    let app = Router::new()
        .route("/request", post(claim));

    // init listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6003")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


async fn claim(Json(param): Json<Claim>) -> Result<Json<ClaimRes>, FaucetError> {

    let network = param.network.clone();

    if network.as_str() != "devnet" && network.as_str() != "testnet" {
        return Err(FaucetError::ParamError(String::from("network must be devnet or testnet")));
    }

    let rpc = format!("{}{}{}", "https://api.", param.network, ".solana.com");
    let client = RpcClient::new(rpc);
    
    let pubkey_vec = bs58::decode(param.address.clone())
    .into_vec()
    .map_err(|_err| FaucetError::InvalidPubkey(param.address))?;

    let arr: [u8; 32] = pubkey_vec.try_into().unwrap();
 
    let pubkey: Pubkey = Pubkey::new_from_array(arr);
    let mul_coefficient: f64 = 10.0_f64.powi(9);
    let lamports = (param.amount * mul_coefficient) as u64;
    let res = client.request_airdrop(&pubkey, lamports)
    .map_err(|err| FaucetError::TransactionErr(err.to_string()))?;
    tracing::info!("Array: {:?}", res);
    Ok(Json(ClaimRes {
        success: true,
        tx_id: res.to_string(),
        explorer_url: String::from("https://solscan.io/tx/") + res.to_string().as_str() + "?cluster=" + network.as_str()
    }))
}

///
/// param struct
#[derive(Deserialize)]
struct Claim {
    network: String,
    address: String,
    amount: f64
}

///
/// res struct
#[derive(Serialize)]
struct ClaimRes {
    success: bool,
    tx_id: String,
    explorer_url: String,
}
