#[cfg(test)]
mod nfts {
    use std::str::FromStr;
    use helius_rust_client::{client::{init::HeliusClient}, models::{nft::{MintListRequestConfig, ActiveListingsRequestConfig}, enriched_transaction::RequestConfig}};
    use solana_program::pubkey::Pubkey;
    use solana_sdk::commitment_config::CommitmentLevel;
    use tokio;

    #[tokio::test]
    async fn get_nft_events() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );
        let config = RequestConfig {
            address: Pubkey::from_str("YourAddress").unwrap(),
            source: None,
            transaction_type: None,
            before: None,
            until: None,
            limit: Some(20),
            commitment: Some(CommitmentLevel::Confirmed),
        };
        let x = client.get_nft_events(config).await;

        println!("{:?}", x.unwrap());
    }


    #[tokio::test]
    async fn get_mint_list() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );

        let config = MintListRequestConfig {
            verified_collection_addresses: None,
            first_verified_creators: Some(vec![
                "A4FM6h8T5Fmh9z2g3fKUrKfZn6BNFEgByR8QGpdbQhk1".to_string()
            ]),
            limit: None,
            pagination_token: None,
        };

        let x = client.get_mint_list(config).await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn get_active_nft_listings() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );

        let config = ActiveListingsRequestConfig {
            marketplaces: vec!["MAGIC_EDEN".to_string()],
            first_verified_creators: Some(vec![
                "A4FM6h8T5Fmh9z2g3fKUrKfZn6BNFEgByR8QGpdbQhk1".to_string()
            ]),
            verified_collection_addresses: None,
            limit: Some(20),
            pagination_token: None,
        };

        let x = client.get_active_nft_listings(config).await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn get_tokens_metadata() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );
        let x = client
            .get_tokens_metadata(vec![
                "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
            ])
            .await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn get_token_balances() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );
        let x = client
            .get_token_balances("YourAddress".to_string())
            .await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn get_nfts_metadata() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );
        let x = client
            .get_nfts_metadata(vec![
                "NftAddress".to_string()
            ])
            .await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn get_nfts_for_address() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );
        let x = client
            .get_nfts(
                "YourAddress".to_string(),
                Some(1),
            )
            .await;

        println!("{:?}", x.unwrap());
    }
}
