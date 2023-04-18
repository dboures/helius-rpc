use crate::models::transactions::UiTransactionStatusMeta;

use super::helius_rpc::{HeliusRpcClient, API_URL_V0};
use serde::Deserialize;
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};
use solana_program::{clock::UnixTimestamp, slot_history::Slot, pubkey::Pubkey};
use solana_sdk::{
    commitment_config::{CommitmentLevel},
    signature::Signature,
};
use solana_transaction_status::{UiTransaction};

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusTxn {
    pub slot: Slot,
    pub block_time: Option<UnixTimestamp>,
    pub transaction: UiTransaction,
    pub meta: Option<UiTransactionStatusMeta>,
}

#[derive(Debug, Default)]
pub struct GetRawTransactionsRequestConfig {
    pub address: Pubkey,
    pub before: Option<Signature>,
    pub until: Option<Signature>,
    pub limit: Option<usize>,
    pub commitment: Option<CommitmentLevel>,
}
impl GetRawTransactionsRequestConfig {
    fn generate_query_parameters(&self, api_key: String) -> ClientResult<Vec<(String, String)>> {
        let mut query_params = vec![("address".to_string(), self.address.to_string()), ("api-key".to_string(), api_key)];
        if self.before.is_some() {
            query_params.push(("before".to_string(), self.before.unwrap().to_string()));
        }
        if self.until.is_some() {
            query_params.push(("until".to_string(), self.until.unwrap().to_string()));
        }
        if self.limit.is_some() {
            query_params.push(("limit".to_string(), self.limit.unwrap().to_string()));
        }

        match self.commitment {
            Some(CommitmentLevel::Confirmed) => {
                query_params.push(("commitment".to_string(), "confirmed".to_string()));
            }
            Some(CommitmentLevel::Finalized) => {
                query_params.push(("commitment".to_string(), "finalized".to_string()));
            }
            _ => {
                return Err(ClientError::from(ClientErrorKind::Custom(
                    "Only Confirmed and Finalized commitments are supported by this API"
                        .to_string(),
                )));
            }
        }
        Ok(query_params)
    }
}

impl HeliusRpcClient {
    /// Returns raw transaction history for a given address. Calls `https://api.helius.xyz/v0/addresses/{address}/raw-transactions`.
    /// * `address` - The addresses that you want transactions for.
    pub async fn get_raw_transactions(
        &self,
        config: GetRawTransactionsRequestConfig,
    ) -> ClientResult<Vec<HeliusTxn>> {
            let query = config.generate_query_parameters(self.api_key.clone())?;
            let request_url = format!(
                "{}/addresses/{}/raw-transactions?",
                API_URL_V0, config.address.to_string(), 
            );

            let res: Vec<HeliusTxn> = self
                .rest_client
                .get(request_url)
                .query(&query)
                .send()
                .await?
                .json()
                .await?;

            println!("{:?}", res);
            Ok(vec![])
        // TODO: enriched Transactions
    }
}
