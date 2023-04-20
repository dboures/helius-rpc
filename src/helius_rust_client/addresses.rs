use super::helius_rust_client::{HeliusClient, API_URL_V0};
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
pub struct NftResponse {
    pub number_of_pages: usize,
    pub nfts: NftInfo,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename = "nft")]
#[serde(rename_all = "camelCase")]
pub struct NftInfo {
    pub name: String,
    pub token_address: String,
    pub collection_address: String,
    pub collection_name: String,
    pub image_url: String,
    pub traits: Vec<Trait>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Trait {
    pub trait_type: String,
    pub value: String,
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

impl HeliusClient {
    /// Returns the Solana Naming Service name for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/names`.
    /// * `address` - The addresses that you want names for.
    pub async fn get_naming_service_names(&self, address: String) -> ClientResult<Vec<String>> {
        let request_url = format!(
            "{}/addresses/{}/names?api-key={}",
            API_URL_V0, address, self.api_key
        );

        let res: DomainNames = self
            .http_client
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
            .http_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }

    /// Returns the NFTs held for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/nfts`.
    /// * `address` - The addresses that you want nfts for.
    pub async fn get_nfts(
        &self,
        address: String,
        page_number: Option<usize>,
    ) -> ClientResult<NftResponse> {
        let mut request_url = format!(
            "{}/addresses/{}/nfts?api-key={}",
            API_URL_V0, address, self.api_key
        );

        if page_number.is_some() {
            request_url = format!("{}&pageNumber={}", request_url, page_number.unwrap());
        }

        let res: NftResponse = self
            .http_client
            .get(request_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        Ok(res)
    }
}
