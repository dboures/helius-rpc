#[cfg(test)]
mod webhooks {
    use helius_rust_client::{client::{init::HeliusClient, webhooks::{WebhookType, CreateWebhookRequest}}, models::enums::TransactionType};
    use tokio;

    #[tokio::test]
    async fn create_webhook() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );

        let example_hook_request = CreateWebhookRequest {
            webhook_url: "https://discord.com/api/webhooks/12345".to_string(),
            transaction_types: vec![TransactionType::NFT_BID],
            account_addresses: vec!["YourAddress".to_string()],
            webhook_type: WebhookType::discord,
            auth_header: "HEADER".to_owned(),
        };
        let x = client.create_webhook(example_hook_request).await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn get_webhooks() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );

        let x = client.get_webhooks().await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn get_webhook() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );

        let x = client
            .get_webhook("webhook-id".to_string())
            .await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn edit_webhook() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );
        let goo = CreateWebhookRequest {
            webhook_url: "https://discord.com/api/webhooks/12345".to_string(),
            transaction_types: vec![TransactionType::NFT_AUCTION_CANCELLED],
            account_addresses: vec!["YourAddress".to_string()],
            webhook_type: WebhookType::discord,
            auth_header: "HEADER".to_owned(),
        };
        let x = client
            .edit_webhook("webhook-id".to_string(), goo)
            .await;

        println!("{:?}", x.unwrap());
    }

    #[tokio::test]
    async fn delete_webhook() {
        let client = HeliusClient::new(
            "your-api-key".to_string(),
            solana_sdk::genesis_config::ClusterType::MainnetBeta,
        );
        let x = client
            .delete_webhook("webhook-id".to_string())
            .await;

        println!("{:?}", x.unwrap());
    }


}

