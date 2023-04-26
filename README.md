# Helius Rust Client

Rust client for Helius's Solana APIs and standard Solana RPC calls. Read more about Helius's APIs [here](https://docs.helius.xyz/welcome/what-is-helius).

Available on [crates.io](https://crates.io/crates/helius-rust-client).

## Install

Add the crate to your `Cargo.toml`.

```
helius-rust-client = "0.1.0"
```

### Examples

```rust
   
    let client = HeliusClient::new(
        "your-api-key".to_string(),
        solana_sdk::genesis_config::ClusterType::MainnetBeta,
    );
    let token_balances = client
        .get_token_balances("YourAddress".to_string())
        .await.unwrap();

    println!("token balances: {:?}", token_balances);
    
```

More examples can be found in the [`tests`] directory (in the form of tests). Provide an API key and addresses and run them with ```-- --nocapture``` in order to see the printed outputs.