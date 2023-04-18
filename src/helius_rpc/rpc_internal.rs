use std::time::Duration;

use delegate::delegate;
use serde;
use serde_json::Value;
use solana_account_decoder::parse_token::{UiTokenAccount, UiTokenAmount};
#[allow(deprecated)]
use solana_client::{
    client_error::Result as ClientResult,
    nonblocking::rpc_client::RpcClient,
    rpc_client::{
        GetConfirmedSignaturesForAddress2Config, Mocks, RpcClientConfig, SerializableMessage,
        SerializableTransaction,
    },
    rpc_config::{
        RpcAccountInfoConfig, RpcBlockConfig, RpcBlockProductionConfig, RpcGetVoteAccountsConfig,
        RpcLargestAccountsConfig, RpcLeaderScheduleConfig, RpcProgramAccountsConfig,
        RpcRequestAirdropConfig, RpcSendTransactionConfig, RpcSimulateTransactionConfig,
        RpcTransactionConfig,
    },
    rpc_deprecated_config::{RpcConfirmedBlockConfig, RpcConfirmedTransactionConfig},
    rpc_request::{RpcRequest, TokenAccountsFilter},
    rpc_response::{
        Fees, RpcAccountBalance, RpcBlockProduction, RpcConfirmedTransactionStatusWithSignature,
        RpcContactInfo, RpcInflationGovernor, RpcInflationRate, RpcInflationReward,
        RpcKeyedAccount, RpcLeaderSchedule, RpcPerfSample, RpcPrioritizationFee, RpcResult,
        RpcSimulateTransactionResult, RpcSnapshotSlotInfo, RpcStakeActivation, RpcSupply,
        RpcVersionInfo, RpcVoteAccountStatus,
    },
    rpc_sender::{RpcSender, RpcTransportStats},
};
use solana_program::{
    clock::UnixTimestamp,
    epoch_schedule::EpochSchedule,
    fee_calculator::{FeeCalculator, FeeRateGovernor},
    hash::Hash,
};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig, epoch_info::EpochInfo,
    genesis_config::ClusterType, pubkey::Pubkey, signature::Signature, slot_history::Slot,
    stake_history::Epoch, transaction,
};
use solana_transaction_status::{
    EncodedConfirmedBlock, EncodedConfirmedTransactionWithStatusMeta, TransactionStatus,
    UiConfirmedBlock, UiTransactionEncoding,
};

use super::helius_rpc::{HeliusRpcClient, DEVNET_RPC_URL, MAINNET_RPC_URL};

#[allow(deprecated)]
impl HeliusRpcClient {
    pub fn new(api_key: String, cluster_type: ClusterType) -> Self {
        let url = match cluster_type {
            ClusterType::Testnet => panic!("Testnet cluster not supported"),
            ClusterType::MainnetBeta => format!("{}{}", MAINNET_RPC_URL, api_key),
            ClusterType::Devnet => format!("{}{}", DEVNET_RPC_URL, api_key),
            ClusterType::Development => panic!("Local cluster not supported"), // TODO
        };
        HeliusRpcClient {
            rpc_client: RpcClient::new(url),
            cluster: cluster_type,
            api_key,
            rest_client: reqwest::Client::new(),
        }
    }

    // pub fn new_with_commitment(url: String, commitment_config: CommitmentConfig) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_with_commitment(url, commitment_config),
    //     }
    // }

    // pub fn new_with_timeout(url: String, timeout: Duration) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_with_timeout(url, timeout),
    //     }
    // }

    // pub fn new_with_timeout_and_commitment(
    //     url: String,
    //     timeout: Duration,
    //     commitment_config: CommitmentConfig,
    // ) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_with_timeout_and_commitment(url, timeout, commitment_config),
    //     }
    // }

    // pub fn new_with_timeouts_and_commitment(
    //     url: String,
    //     timeout: Duration,
    //     commitment_config: CommitmentConfig,
    //     confirm_transaction_initial_timeout: Duration,
    // ) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_with_timeouts_and_commitment(
    //             url,
    //             timeout,
    //             commitment_config,
    //             confirm_transaction_initial_timeout,
    //         ),
    //     }
    // }

    // pub fn new_mock(url: String) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_mock(url),
    //     }
    // }

    // pub fn new_mock_with_mocks(url: String, mocks: Mocks) -> Self {
    //     HeliusRpcClient {
    //         rpc_client: RpcClient::new_mock_with_mocks(url, mocks),
    //     }
    // }

    delegate! {
        to self.rpc_client {
            pub fn url(&self) -> String;
            pub fn commitment(&self) -> CommitmentConfig;
            pub async fn send_and_confirm_transaction(
                &self,
                transaction: &impl SerializableTransaction,
            ) -> ClientResult<Signature>;
            pub async fn send_and_confirm_transaction_with_spinner(
                &self,
                transaction: &impl SerializableTransaction,
            ) -> ClientResult<Signature>;
            pub async fn send_and_confirm_transaction_with_spinner_and_commitment(
                &self,
                transaction: &impl SerializableTransaction,
                commitment: CommitmentConfig,
            ) -> ClientResult<Signature>;
            pub async fn send_and_confirm_transaction_with_spinner_and_config(
                &self,
                transaction: &impl SerializableTransaction,
                commitment: CommitmentConfig,
                config: RpcSendTransactionConfig,
            ) -> ClientResult<Signature>;
            pub async fn send_transaction(
                &self,
                transaction: &impl SerializableTransaction,
            ) -> ClientResult<Signature>;
            pub async fn send_transaction_with_config(
                &self,
                transaction: &impl SerializableTransaction,
                config: RpcSendTransactionConfig,
            ) -> ClientResult<Signature>;
            pub async fn confirm_transaction(&self, signature: &Signature) -> ClientResult<bool>;
            pub async fn confirm_transaction_with_commitment(
                &self,
                signature: &Signature,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<bool>;
            pub async fn confirm_transaction_with_spinner(
                &self,
                signature: &Signature,
                recent_blockhash: &Hash,
                commitment: CommitmentConfig,
            ) -> ClientResult<()>;
            pub async fn simulate_transaction(
                &self,
                transaction: &impl SerializableTransaction,
            ) -> RpcResult<RpcSimulateTransactionResult>;
            pub async fn simulate_transaction_with_config(
                &self,
                transaction: &impl SerializableTransaction,
                config: RpcSimulateTransactionConfig,
            ) -> RpcResult<RpcSimulateTransactionResult>;
            pub async fn get_highest_snapshot_slot(&self) -> ClientResult<RpcSnapshotSlotInfo>;
            pub async fn get_snapshot_slot(&self) -> ClientResult<Slot>;
            pub async fn get_signature_status(
                &self,
                signature: &Signature,
            ) -> ClientResult<Option<transaction::Result<()>>>;
            pub async fn get_signature_statuses(
                &self,
                signatures: &[Signature],
            ) -> RpcResult<Vec<Option<TransactionStatus>>>;
            pub async fn get_signature_statuses_with_history(
                &self,
                signatures: &[Signature],
            ) -> RpcResult<Vec<Option<TransactionStatus>>>;
            pub async fn get_signature_status_with_commitment(
                &self,
                signature: &Signature,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<Option<transaction::Result<()>>>;
            pub async fn get_signature_status_with_commitment_and_history(
                &self,
                signature: &Signature,
                commitment_config: CommitmentConfig,
                search_transaction_history: bool,
            ) -> ClientResult<Option<transaction::Result<()>>>;
            pub async fn get_slot(&self) -> ClientResult<Slot>;
            pub async fn get_slot_with_commitment(
                &self,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<Slot>;
            pub async fn get_block_height(&self) -> ClientResult<u64>;
            pub async fn get_block_height_with_commitment(
                &self,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<u64>;
            pub async fn get_slot_leaders(
                &self,
                start_slot: Slot,
                limit: u64,
            ) -> ClientResult<Vec<Pubkey>>;
            pub async fn get_block_production(&self) -> RpcResult<RpcBlockProduction>;
            pub async fn get_block_production_with_config(
                &self,
                config: RpcBlockProductionConfig,
            ) -> RpcResult<RpcBlockProduction>;
            pub async fn get_stake_activation(
                &self,
                stake_account: Pubkey,
                epoch: Option<Epoch>,
            ) -> ClientResult<RpcStakeActivation>;
            pub async fn supply(&self) -> RpcResult<RpcSupply>;
            pub async fn supply_with_commitment(
                &self,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<RpcSupply>;
            pub async fn get_largest_accounts_with_config(
                &self,
                config: RpcLargestAccountsConfig,
            ) -> RpcResult<Vec<RpcAccountBalance>>;
            pub async fn get_vote_accounts(&self) -> ClientResult<RpcVoteAccountStatus>;
            pub async fn get_vote_accounts_with_commitment(
                &self,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<RpcVoteAccountStatus>;
            pub async fn get_vote_accounts_with_config(
                &self,
                config: RpcGetVoteAccountsConfig,
            ) -> ClientResult<RpcVoteAccountStatus>;
            pub async fn wait_for_max_stake(
                &self,
                commitment: CommitmentConfig,
                max_stake_percent: f32,
            ) -> ClientResult<()>;
            pub async fn get_cluster_nodes(&self) -> ClientResult<Vec<RpcContactInfo>>;
            pub async fn get_block(&self, slot: Slot) -> ClientResult<EncodedConfirmedBlock>;
            pub async fn get_block_with_encoding(
                &self,
                slot: Slot,
                encoding: UiTransactionEncoding,
            ) -> ClientResult<EncodedConfirmedBlock>;
            pub async fn get_block_with_config(
                &self,
                slot: Slot,
                config: RpcBlockConfig,
            ) -> ClientResult<UiConfirmedBlock>;
            pub async fn get_confirmed_block(&self, slot: Slot) -> ClientResult<EncodedConfirmedBlock>;
            pub async fn get_confirmed_block_with_encoding(
                &self,
                slot: Slot,
                encoding: UiTransactionEncoding,
            ) -> ClientResult<EncodedConfirmedBlock>;
            pub async fn get_confirmed_block_with_config(
                &self,
                slot: Slot,
                config: RpcConfirmedBlockConfig,
            ) -> ClientResult<UiConfirmedBlock>;
            pub async fn get_blocks(
                &self,
                start_slot: Slot,
                end_slot: Option<Slot>,
            ) -> ClientResult<Vec<Slot>>;
            pub async fn get_blocks_with_commitment(
                &self,
                start_slot: Slot,
                end_slot: Option<Slot>,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<Vec<Slot>>;
            pub async fn get_blocks_with_limit(
                &self,
                start_slot: Slot,
                limit: usize,
            ) -> ClientResult<Vec<Slot>>;
            pub async fn get_blocks_with_limit_and_commitment(
                &self,
                start_slot: Slot,
                limit: usize,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<Vec<Slot>>;
            pub async fn get_confirmed_blocks(
                &self,
                start_slot: Slot,
                end_slot: Option<Slot>,
            ) -> ClientResult<Vec<Slot>>;
            pub async fn get_confirmed_blocks_with_commitment(
                &self,
                start_slot: Slot,
                end_slot: Option<Slot>,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<Vec<Slot>>;
            pub async fn get_confirmed_blocks_with_limit(
                &self,
                start_slot: Slot,
                limit: usize,
            ) -> ClientResult<Vec<Slot>>;
            pub async fn get_confirmed_blocks_with_limit_and_commitment(
                &self,
                start_slot: Slot,
                limit: usize,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<Vec<Slot>>;
            pub async fn get_signatures_for_address(
                &self,
                address: &Pubkey,
            ) -> ClientResult<Vec<RpcConfirmedTransactionStatusWithSignature>>;
            pub async fn get_signatures_for_address_with_config(
                &self,
                address: &Pubkey,
                config: GetConfirmedSignaturesForAddress2Config,
            ) -> ClientResult<Vec<RpcConfirmedTransactionStatusWithSignature>>;
            pub async fn get_confirmed_signatures_for_address2(
                &self,
                address: &Pubkey,
            ) -> ClientResult<Vec<RpcConfirmedTransactionStatusWithSignature>>;
            pub async fn get_confirmed_signatures_for_address2_with_config(
                &self,
                address: &Pubkey,
                config: GetConfirmedSignaturesForAddress2Config,
            ) -> ClientResult<Vec<RpcConfirmedTransactionStatusWithSignature>>;
            pub async fn get_transaction(
                &self,
                signature: &Signature,
                encoding: UiTransactionEncoding,
            ) -> ClientResult<EncodedConfirmedTransactionWithStatusMeta>;
            pub async fn get_transaction_with_config(
                &self,
                signature: &Signature,
                config: RpcTransactionConfig,
            ) -> ClientResult<EncodedConfirmedTransactionWithStatusMeta>;
            pub async fn get_confirmed_transaction(
                &self,
                signature: &Signature,
                encoding: UiTransactionEncoding,
            ) -> ClientResult<EncodedConfirmedTransactionWithStatusMeta>;
            pub async fn get_confirmed_transaction_with_config(
                &self,
                signature: &Signature,
                config: RpcConfirmedTransactionConfig,
            ) -> ClientResult<EncodedConfirmedTransactionWithStatusMeta>;
            pub async fn get_block_time(&self, slot: Slot) -> ClientResult<UnixTimestamp>;
            pub async fn get_epoch_info(&self) -> ClientResult<EpochInfo>;
            pub async fn get_epoch_info_with_commitment(
                &self,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<EpochInfo>;
            pub async fn get_leader_schedule(
                &self,
                slot: Option<Slot>,
            ) -> ClientResult<Option<RpcLeaderSchedule>>;
            pub async fn get_leader_schedule_with_commitment(
                &self,
                slot: Option<Slot>,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<Option<RpcLeaderSchedule>>;
            pub async fn get_leader_schedule_with_config(
                &self,
                slot: Option<Slot>,
                config: RpcLeaderScheduleConfig,
            ) -> ClientResult<Option<RpcLeaderSchedule>>;
            pub async fn get_epoch_schedule(&self) -> ClientResult<EpochSchedule>;
            pub async fn get_recent_performance_samples(
                &self,
                limit: Option<usize>,
            ) -> ClientResult<Vec<RpcPerfSample>>;
            pub async fn get_recent_prioritization_fees(
                &self,
                addresses: &[Pubkey],
            ) -> ClientResult<Vec<RpcPrioritizationFee>>;
            pub async fn get_identity(&self) -> ClientResult<Pubkey>;
            pub async fn get_inflation_governor(&self) -> ClientResult<RpcInflationGovernor>;
            pub async fn get_inflation_rate(&self) -> ClientResult<RpcInflationRate>;
            pub async fn get_inflation_reward(
                &self,
                addresses: &[Pubkey],
                epoch: Option<Epoch>,
            ) -> ClientResult<Vec<Option<RpcInflationReward>>>;
            pub async fn get_version(&self) -> ClientResult<RpcVersionInfo>;
            pub async fn minimum_ledger_slot(&self) -> ClientResult<Slot>;
            pub async fn get_account(&self, pubkey: &Pubkey) -> ClientResult<Account>;
            pub async fn get_account_with_commitment(
                &self,
                pubkey: &Pubkey,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<Option<Account>>;
            pub async fn get_account_with_config(
                &self,
                pubkey: &Pubkey,
                config: RpcAccountInfoConfig,
            ) -> RpcResult<Option<Account>>;
            pub async fn get_max_retransmit_slot(&self) -> ClientResult<Slot>;
            pub async fn get_max_shred_insert_slot(&self) -> ClientResult<Slot>;
            pub async fn get_multiple_accounts(
                &self,
                pubkeys: &[Pubkey],
            ) -> ClientResult<Vec<Option<Account>>>;
            pub async fn get_multiple_accounts_with_commitment(
                &self,
                pubkeys: &[Pubkey],
                commitment_config: CommitmentConfig,
            ) -> RpcResult<Vec<Option<Account>>>;
            pub async fn get_multiple_accounts_with_config(
                &self,
                pubkeys: &[Pubkey],
                config: RpcAccountInfoConfig,
            ) -> RpcResult<Vec<Option<Account>>>;
            pub async fn get_account_data(&self, pubkey: &Pubkey) -> ClientResult<Vec<u8>>;
            pub async fn get_minimum_balance_for_rent_exemption(
                &self,
                data_len: usize,
            ) -> ClientResult<u64>;
            pub async fn get_balance(&self, pubkey: &Pubkey) -> ClientResult<u64>;
            pub async fn get_balance_with_commitment(
                &self,
                pubkey: &Pubkey,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<u64>;
            pub async fn get_program_accounts(
                &self,
                pubkey: &Pubkey,
            ) -> ClientResult<Vec<(Pubkey, Account)>>;
            pub async fn get_program_accounts_with_config(
                &self,
                pubkey: &Pubkey,
                config: RpcProgramAccountsConfig,
            ) -> ClientResult<Vec<(Pubkey, Account)>>;
            pub async fn get_stake_minimum_delegation(&self) -> ClientResult<u64>;
            pub async fn get_stake_minimum_delegation_with_commitment(
                &self,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<u64>;
            pub async fn get_transaction_count(&self) -> ClientResult<u64>;
            pub async fn get_transaction_count_with_commitment(
                &self,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<u64>;
            pub async fn get_fees(&self) -> ClientResult<Fees>;
            pub async fn get_fees_with_commitment(
                &self,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<Fees>;
            pub async fn get_recent_blockhash(&self) -> ClientResult<(Hash, FeeCalculator)>;
            pub async fn get_recent_blockhash_with_commitment(
                &self,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<(Hash, FeeCalculator, Slot)>;
            pub async fn get_fee_calculator_for_blockhash(
                &self,
                blockhash: &Hash,
            ) -> ClientResult<Option<FeeCalculator>>;
            pub async fn get_fee_calculator_for_blockhash_with_commitment(
                &self,
                blockhash: &Hash,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<Option<FeeCalculator>>;
            pub async fn get_fee_rate_governor(&self) -> RpcResult<FeeRateGovernor>;
            pub async fn get_new_blockhash(&self, blockhash: &Hash) -> ClientResult<(Hash, FeeCalculator)>;
            pub async fn get_first_available_block(&self) -> ClientResult<Slot>;
            pub async fn get_genesis_hash(&self) -> ClientResult<Hash>;
            pub async fn get_health(&self) -> ClientResult<()>;
            pub async fn get_token_account(&self, pubkey: &Pubkey) -> ClientResult<Option<UiTokenAccount>>;
            pub async fn get_token_account_with_commitment(
                &self,
                pubkey: &Pubkey,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<Option<UiTokenAccount>>;
            pub async fn get_token_account_balance(&self, pubkey: &Pubkey) -> ClientResult<UiTokenAmount>;
            pub async fn get_token_account_balance_with_commitment(
                &self,
                pubkey: &Pubkey,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<UiTokenAmount>;
            pub async fn get_token_accounts_by_delegate(
                &self,
                delegate: &Pubkey,
                token_account_filter: TokenAccountsFilter,
            ) -> ClientResult<Vec<RpcKeyedAccount>>;
            pub async fn get_token_accounts_by_delegate_with_commitment(
                &self,
                delegate: &Pubkey,
                token_account_filter: TokenAccountsFilter,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<Vec<RpcKeyedAccount>>;
            pub async fn get_token_accounts_by_owner(
                &self,
                owner: &Pubkey,
                token_account_filter: TokenAccountsFilter,
            ) -> ClientResult<Vec<RpcKeyedAccount>>;
            pub async fn get_token_accounts_by_owner_with_commitment(
                &self,
                owner: &Pubkey,
                token_account_filter: TokenAccountsFilter,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<Vec<RpcKeyedAccount>>;
            pub async fn get_token_supply(&self, mint: &Pubkey) -> ClientResult<UiTokenAmount>;
            pub async fn get_token_supply_with_commitment(
                &self,
                mint: &Pubkey,
                commitment_config: CommitmentConfig,
            ) -> RpcResult<UiTokenAmount>;
            pub async fn request_airdrop(&self, pubkey: &Pubkey, lamports: u64) -> ClientResult<Signature>;
            pub async fn request_airdrop_with_blockhash(
                &self,
                pubkey: &Pubkey,
                lamports: u64,
                recent_blockhash: &Hash,
            ) -> ClientResult<Signature>;
            pub async fn request_airdrop_with_config(
                &self,
                pubkey: &Pubkey,
                lamports: u64,
                config: RpcRequestAirdropConfig,
            ) -> ClientResult<Signature>;
            pub async fn poll_get_balance_with_commitment(
                &self,
                pubkey: &Pubkey,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<u64>;
            pub async fn wait_for_balance_with_commitment(
                &self,
                pubkey: &Pubkey,
                expected_balance: Option<u64>,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<u64>;
            pub async fn poll_for_signature(&self, signature: &Signature) -> ClientResult<()>;
            pub async fn poll_for_signature_with_commitment(
                &self,
                signature: &Signature,
                commitment_config: CommitmentConfig,
            ) -> ClientResult<()>;
            pub async fn poll_for_signature_confirmation(
                &self,
                signature: &Signature,
                min_confirmed_blocks: usize,
            ) -> ClientResult<usize>;
            pub async fn get_num_blocks_since_signature_confirmation(
                &self,
                signature: &Signature,
            ) -> ClientResult<usize>;
            pub async fn get_latest_blockhash(&self) -> ClientResult<Hash>;
            pub async fn get_latest_blockhash_with_commitment(
                &self,
                commitment: CommitmentConfig,
            ) -> ClientResult<(Hash, u64)>;
            pub async fn is_blockhash_valid(
                &self,
                blockhash: &Hash,
                commitment: CommitmentConfig,
            ) -> ClientResult<bool>;
            pub async fn get_fee_for_message(
                &self,
                message: &impl SerializableMessage,
            ) -> ClientResult<u64>;
            pub async fn get_new_latest_blockhash(&self, blockhash: &Hash) -> ClientResult<Hash>;
            pub async fn send<T: for<'de> serde::de::Deserialize<'de>>(&self, request: RpcRequest, params: Value) -> ClientResult<T>;
            pub fn get_transport_stats(&self) -> RpcTransportStats;
        }
    }
}
