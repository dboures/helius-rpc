use reqwest::Client as RestClient;
use serde_json::Value;
use solana_client::client_error::Result as ClientResult;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::genesis_config::ClusterType;
use std::collections::HashMap;

use crate::helius_rpc::structs::{TokenMetadata};

pub const API_URL_V0: &str = "https://api.helius.xyz/v0";
pub const API_URL_V1: &str = "https://api.helius.xyz/v1";
pub const MAINNET_RPC_URL: &str = "https://rpc.helius.xyz/?api-key=";
pub const DEVNET_RPC_URL: &str = "https://rpc-devnet.helius.xyz/?api-key=";

pub struct HeliusRpcClient {
    pub rpc_client: RpcClient,
    pub cluster: ClusterType,
    pub(crate) api_key: String,
    pub(crate) rest_client: RestClient,
}

#[allow(deprecated)]
impl HeliusRpcClient {
    pub fn new(api_key: String, cluster_type: ClusterType) -> Self {
        let url = match cluster_type {
            ClusterType::Testnet => panic!("Testnet cluster not supported"),
            ClusterType::MainnetBeta => format!("{}{}", MAINNET_RPC_URL, api_key),
            ClusterType::Devnet => format!("{}{}", DEVNET_RPC_URL, api_key),
            ClusterType::Development => panic!("Local cluster not supported"), // TODO
        };
        HeliusRpcClient {
            rpc_client: RpcClient::new(url),
            cluster: cluster_type,
            api_key,
            rest_client: reqwest::Client::new(),
        }
    }

    // pub fn new_with_commitment(url: String, commitment_config: CommitmentConfig) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_with_commitment(url, commitment_config),
    //     }
    // }

    // pub fn new_with_timeout(url: String, timeout: Duration) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_with_timeout(url, timeout),
    //     }
    // }

    // pub fn new_with_timeout_and_commitment(
    //     url: String,
    //     timeout: Duration,
    //     commitment_config: CommitmentConfig,
    // ) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_with_timeout_and_commitment(url, timeout, commitment_config),
    //     }
    // }

    // pub fn new_with_timeouts_and_commitment(
    //     url: String,
    //     timeout: Duration,
    //     commitment_config: CommitmentConfig,
    //     confirm_transaction_initial_timeout: Duration,
    // ) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_with_timeouts_and_commitment(
    //             url,
    //             timeout,
    //             commitment_config,
    //             confirm_transaction_initial_timeout,
    //         ),
    //     }
    // }

    // pub fn new_mock(url: String) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_mock(url),
    //     }
    // }

    // pub fn new_mock_with_mocks(url: String, mocks: Mocks) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_mock_with_mocks(url, mocks),
    //     }
    // }
}
