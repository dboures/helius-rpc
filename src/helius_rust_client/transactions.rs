use crate::models::transactions::{
    GetRawTransactionsRequestConfig, GetTransactionsRequestConfig, UiTransactionStatusMeta,
};

use super::helius_rust_client::{HeliusClient, API_URL_V0};
use serde::Deserialize;
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};
use solana_program::{clock::UnixTimestamp, pubkey::Pubkey, slot_history::Slot};
use solana_sdk::{commitment_config::CommitmentLevel, signature::Signature};
use solana_transaction_status::UiTransaction;

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusTxn {
    pub slot: Slot,
    pub block_time: Option<UnixTimestamp>,
    pub transaction: UiTransaction,
    pub meta: Option<UiTransactionStatusMeta>,
}

impl HeliusClient {
    /// Returns raw transaction history for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/raw-transactions`.
    /// * `address` - The address that you want transactions for.
    pub async fn get_raw_transactions(
        &self,
        config: GetRawTransactionsRequestConfig,
    ) -> ClientResult<Vec<HeliusTxn>> {
        let query = config.generate_query_parameters(self.api_key.clone())?;
        let request_url = format!(
            "{}/addresses/{}/raw-transactions?",
            API_URL_V0,
            config.address.to_string(),
        );

        let res: Vec<HeliusTxn> = self
            .http_client
            .get(request_url)
            .query(&query)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }

    // // TODO

    // /// Returns enriched transaction history for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/transactions`.
    // /// * `address` - The address that you want transactions for.
    // pub async fn get_transactions(
    //     &self,
    //     config: GetTransactionsRequestConfig,
    // ) -> ClientResult<Vec<HeliusTxn>> {
    //         let query = config.generate_query_parameters(self.api_key.clone())?;
    //         let request_url = format!(
    //             "{}/addresses/{}/transactions?",
    //             API_URL_V0, config.address.to_string(),
    //         );

    //         let res: serde_json::Value = self
    //             .http_client
    //             .get(request_url)
    //             .query(&query)
    //             .send()
    //             .await?
    //             .json()
    //             .await?;

    //         println!("{:?}", res);
    //         Ok(vec![])
    // }
}
