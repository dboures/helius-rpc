#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use helius_rust_client::{client::{init::HeliusClient}, models::{raw_transaction::GetRawTransactionsRequestConfig, enums::{TransactionSource}, enriched_transaction::RequestConfig}};
    use solana_program::pubkey::Pubkey;
    use solana_sdk::commitment_config::CommitmentLevel;
    use tokio;

    #[tokio::test]
    async fn get_transactions() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );

        let config = GetRawTransactionsRequestConfig {
            address: Pubkey::from_str("YourAddress").unwrap(),
            before: None,
            until: None,
            limit: Some(2),
            commitment: Some(CommitmentLevel::Confirmed),
        };

        let x = client.get_transactions_for_address(config).await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn get_transactions_by_hash() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );

        let x = client.get_transactions_by_hash(vec!["YourTxnHash".to_string()], None).await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn get_enriched_transactions() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );
        let config = RequestConfig {
            address: Pubkey::from_str("YourAddress").unwrap(),
            before: None,
            until: None,
            limit: Some(5),
            source: Some(TransactionSource::TENSOR),
            transaction_type: None,
            commitment: Some(CommitmentLevel::Confirmed),
        };

        let x = client.get_enriched_transactions(config).await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn get_enriched_transactions_by_hash() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );

        let x = client
            .get_enriched_transactions_by_hash(vec!["YourTxnHash".to_string()], Some(CommitmentLevel::Confirmed))
            .await;

        println!("{:?}", x.unwrap());
    }
}

