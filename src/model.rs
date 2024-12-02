use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse<T> {
  pub jsonrpc: String,
  pub id: u64,
  pub result: Option<T>,
  pub error: Option<RpcError>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
  pub code: i32,
  pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenSupplyResponse {
    pub amount: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
  pub address: String,
  pub supply: u64,
  pub decimals: u8,
}