use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use bs58;
use std::convert::TryInto;
use axum::{
    routing::post,
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};

///
/// DevNet: https://api.devnet.solana.com
/// TestNet: https://api.testnet.solana.com
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build router
    let app = Router::new()
        .route("/claim", post(claim));

    // init listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn claim(Json(param): Json<Claim>) -> impl IntoResponse {
    let client = RpcClient::new(param.rpc);
    
    let pubkey_vec = bs58::decode(param.address).into_vec().unwrap();
    let arr: [u8; 32] = pubkey_vec.try_into().unwrap();
 
    let pubkey = Pubkey::new_from_array(arr);

    let res = client.request_airdrop(&pubkey, param.amount).unwrap();
    println!("Array: {:?}", res);
    (StatusCode::OK, Json(ClaimRes {hash: res.to_string()}))
}

///
/// param struct
#[derive(Deserialize)]
struct Claim {
    rpc: String,
    address: String,
    amount: u64
}

///
/// res struct
#[derive(Serialize)]
struct ClaimRes {
    hash: String
}
