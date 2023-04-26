#[cfg(test)]
mod misc {
    use std::str::FromStr;
    use helius_rust_client::{client::{init::HeliusClient, webhooks::{WebhookType, CreateWebhookRequest}}, models::{raw_transaction::GetRawTransactionsRequestConfig, nft::{MintListRequestConfig, ActiveListingsRequestConfig, NftMetadata}, enums::{TransactionType, TransactionSource}, enriched_transaction::RequestConfig}};
    use solana_program::pubkey::Pubkey;
    use solana_sdk::commitment_config::CommitmentLevel;
    use tokio;

    #[tokio::test]
    async fn get_slot() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );
        let slot = client.rpc_client.get_slot().await;
        assert!(slot.unwrap() > 0);
    }

    #[tokio::test]
    async fn get_naming_service_names() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );
        let x = client
            .get_naming_service_names("YourAddress".to_string())
            .await;

        println!("{:?}", x.unwrap());
    }
}