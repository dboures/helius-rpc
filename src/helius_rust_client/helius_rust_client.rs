use reqwest::Client as RestClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client::Mocks;
use solana_sdk::{commitment_config::CommitmentConfig, genesis_config::ClusterType};
use std::time::Duration;

pub const API_URL_V0: &str = "https://api.helius.xyz/v0";
pub const API_URL_V1: &str = "https://api.helius.xyz/v1";
pub const MAINNET_RPC_URL: &str = "https://rpc.helius.xyz/?api-key=";
pub const DEVNET_RPC_URL: &str = "https://rpc-devnet.helius.xyz/?api-key=";

pub struct HeliusClient {
    pub rpc_client: RpcClient,
    pub http_client: RestClient,
    pub cluster: ClusterType,
    pub(crate) api_key: String,
}

impl HeliusClient {
    pub fn new(api_key: String, cluster_type: ClusterType) -> Self {
        let url = format_url(api_key.clone(), cluster_type);
        HeliusClient {
            rpc_client: RpcClient::new(url),
            cluster: cluster_type,
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn new_with_commitment(
        api_key: String,
        cluster_type: ClusterType,
        commitment_config: CommitmentConfig,
    ) -> Self {
        let url = format_url(api_key.clone(), cluster_type);
        HeliusClient {
            rpc_client: RpcClient::new_with_commitment(url, commitment_config),
            cluster: cluster_type,
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn new_with_timeout(api_key: String, cluster_type: ClusterType, timeout: Duration) -> Self {
        let url = format_url(api_key.clone(), cluster_type);
        HeliusClient {
            rpc_client: RpcClient::new_with_timeout(url, timeout),
            cluster: cluster_type,
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn new_with_timeout_and_commitment(
        api_key: String,
        cluster_type: ClusterType,
        timeout: Duration,
        commitment_config: CommitmentConfig,
    ) -> Self {
        let url = format_url(api_key.clone(), cluster_type);
        HeliusClient {
            rpc_client: RpcClient::new_with_timeout_and_commitment(url, timeout, commitment_config),
            cluster: cluster_type,
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn new_with_timeouts_and_commitment(
        api_key: String,
        cluster_type: ClusterType,
        timeout: Duration,
        commitment_config: CommitmentConfig,
        confirm_transaction_initial_timeout: Duration,
    ) -> Self {
        let url = format_url(api_key.clone(), cluster_type);
        HeliusClient {
            rpc_client: RpcClient::new_with_timeouts_and_commitment(
                url,
                timeout,
                commitment_config,
                confirm_transaction_initial_timeout,
            ),
            cluster: cluster_type,
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn new_mock(api_key: String, cluster_type: ClusterType) -> Self {
        let url = format_url(api_key.clone(), cluster_type);
        HeliusClient {
            rpc_client: RpcClient::new_mock(url),
            cluster: cluster_type,
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn new_mock_with_mocks(api_key: String, cluster_type: ClusterType, mocks: Mocks) -> Self {
        let url = format_url(api_key.clone(), cluster_type);
        HeliusClient {
            rpc_client: RpcClient::new_mock_with_mocks(url, mocks),
            cluster: cluster_type,
            api_key,
            http_client: reqwest::Client::new(),
        }
    }
}

fn format_url(api_key: String, cluster_type: ClusterType) -> String {
    match cluster_type {
        ClusterType::Testnet => panic!("Testnet cluster not supported"),
        ClusterType::MainnetBeta => format!("{}{}", MAINNET_RPC_URL, api_key),
        ClusterType::Devnet => format!("{}{}", DEVNET_RPC_URL, api_key),
        ClusterType::Development => panic!("Local cluster not supported"),
    }
}
