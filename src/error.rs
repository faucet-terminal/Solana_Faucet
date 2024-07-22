use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FaucetError{

    #[error("{0}")]
    ParamError(String),
    #[error("Invalid pubkey: {0}")]
    InvalidPubkey(String),
    #[error("send tx err: {0}")]
    TransactionErr(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct FaucetResult {
    success: bool,
    message: String,
}

impl IntoResponse for FaucetError {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();
        let error_res = FaucetResult {
            success: false,
            message,
        };
        let json = Json(error_res);
        (StatusCode::INTERNAL_SERVER_ERROR, json).into_response()
    }
}