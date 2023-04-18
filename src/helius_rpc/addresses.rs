use super::helius_rpc::{HeliusRpcClient, API_URL_V0};
use serde::Deserialize;
use solana_client::client_error::Result as ClientResult;
use solana_program::{clock::UnixTimestamp, slot_history::Slot};
use solana_transaction_status::{UiTransaction, UiTransactionStatusMeta};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct DomainNames {
    domain_names: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalancesResponse {
    pub native_balance: u64,
    pub tokens: Vec<TokenBalance>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub token_account: String,
    pub mint: String,
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusTxn {
    pub slot: Slot,
    pub block_time: Option<UnixTimestamp>,
    pub transaction: UiTransaction,
    pub meta: Option<UiTransactionStatusMeta>,
}

impl HeliusRpcClient {
    /// Returns the Solana Naming Service name for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/names`.
    /// * `address` - The addresses that you want names for.
    pub async fn get_naming_service_names(&self, address: String) -> ClientResult<Vec<String>> {
        let request_url = format!(
            "{}/addresses/{}/names?api-key={}",
            API_URL_V0, address, self.api_key
        );

        let res: DomainNames = self
            .rest_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json()
            .await?;
        Ok(res.domain_names)
    }

    /// Returns the native balance and token balances for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/balances`.
    /// * `address` - The addresses that you want token balances for.
    pub async fn get_token_balances(&self, address: String) -> ClientResult<TokenBalancesResponse> {
        let request_url = format!(
            "{}/addresses/{}/balances?api-key={}",
            API_URL_V0, address, self.api_key
        );

        let res: TokenBalancesResponse = self
            .rest_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }
}
