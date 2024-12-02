use anyhow::{Result, Context};
use reqwest;
use serde_json;
use tracing::{info, error};

use crate::config::AppConfig;
use crate::error::{AnalyzerError, Result as AppResult};
use crate::model::{RpcRequest, RpcResponse, TokenSupplyResponse, TokenInfo};

pub struct SolanaRpcClient {
    client: reqwest::Client,
    rpc_endpoint: String,
}

impl SolanaRpcClient {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            rpc_endpoint: config.api_endpoints.solana_rpc.clone(),
        }
    }

    pub async fn get_token_supply(&self, token_address: &str) -> AppResult<TokenInfo> {
      info!("Fetching token supply for address: {}", token_address);
  
      // Validate token address (basic check)
      if token_address.len() != 44 {
          return Err(AnalyzerError::invalid_address(token_address));
      }
  

      let request = RpcRequest {
        jsonrpc: "2.0".to_string(),
        id: 1u64,
        method: "getTokenSupply".to_string(),
        params: vec![serde_json::Value::String(token_address.to_string())],
    };
  
      let response = self.client
          .post(&self.rpc_endpoint)
          .json(&request)
          .send()
          .await
          .map_err(|e| {
              error!("RPC request failed: {}", e);
              AnalyzerError::invalid_address(token_address)
          })?;
  
      let rpc_response: RpcResponse<TokenSupplyResponse> = response
          .json()
          .await
          .map_err(|e| {
              error!("Failed to parse RPC response: {}", e);
              AnalyzerError::invalid_address(token_address)
          })?;
  
      let supply_data = rpc_response.result
          .ok_or_else(|| {
              error!("No supply data in response");
              AnalyzerError::invalid_address(token_address)
          })?;
  
      // Convert amount string to u64
      let amount = supply_data.amount
          .parse()
          .map_err(|e| {
              error!("Failed to parse token supply amount: {}", e);
              AnalyzerError::invalid_address(token_address)
          })?;
  
      Ok(TokenInfo {
          address: token_address.to_string(),
          supply: amount,
          decimals: supply_data.decimals,
      })
  }
}