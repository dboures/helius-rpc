use std::fmt::{self, Display};

use serde::Deserialize;
use solana_client::client_error::{ClientError, ClientErrorKind, Result as ClientResult};
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    commitment_config::CommitmentLevel, signature::Signature, transaction::TransactionError,
};
use solana_transaction_status::{
    option_serializer::OptionSerializer, Rewards, UiInnerInstructions, UiLoadedAddresses,
    UiTransactionReturnData, UiTransactionTokenBalance,
};

#[derive(Debug, Default)]
pub struct GetRawTransactionsRequestConfig {
    pub address: Pubkey,
    pub before: Option<Signature>,
    pub until: Option<Signature>,
    pub limit: Option<usize>,
    pub commitment: Option<CommitmentLevel>,
}
impl GetRawTransactionsRequestConfig {
    pub fn generate_query_parameters(
        &self,
        api_key: String,
    ) -> ClientResult<Vec<(String, String)>> {
        let mut query_params = vec![
            ("address".to_string(), self.address.to_string()),
            ("api-key".to_string(), api_key),
        ];
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

#[derive(Debug, Default)]
pub struct GetTransactionsRequestConfig {
    pub address: Pubkey,
    pub before: Option<Signature>,
    pub until: Option<Signature>,
    pub limit: Option<usize>,
    pub source: Option<TransactionSource>,
    pub transaction_type: Option<TransactionType>,
    pub commitment: Option<CommitmentLevel>,
}
impl GetTransactionsRequestConfig {
    pub fn generate_query_parameters(
        &self,
        api_key: String,
    ) -> ClientResult<Vec<(String, String)>> {
        let mut query_params = vec![
            ("address".to_string(), self.address.to_string()),
            ("api-key".to_string(), api_key),
        ];
        if self.before.is_some() {
            query_params.push(("before".to_string(), self.before.unwrap().to_string()));
        }
        if self.until.is_some() {
            query_params.push(("until".to_string(), self.until.unwrap().to_string()));
        }
        if self.limit.is_some() {
            query_params.push(("limit".to_string(), self.limit.unwrap().to_string()));
        }
        if self.source.is_some() {
            query_params.push(("source".to_string(), self.source.unwrap().to_string()));
        }
        if self.transaction_type.is_some() {
            query_params.push((
                "type".to_string(),
                self.transaction_type.unwrap().to_string(),
            ));
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

// #[derive(Clone, Debug, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct NativeTransfer {
//     from_user_account: String,
//     to_user_account: String,
//     amount: u64,
// }

// #[derive(Clone, Debug, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TokenTransfer {
//     pub from_user_account: String,
//     pub to_user_account: String,
//     pub from_token_account: String,
//     pub to_token_account: String,
//     pub token_amount: u64,
//     pub mint: String,
// }

// #[derive(Clone, Debug, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct InnerInstruction {
//     pub accounts: Vec<String>,
//     pub data: String,
//     pub program_id: String,
// }

// #[derive(Clone, Debug, PartialEq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Instruction {
//     pub accounts: Vec<String>,
//     pub data: String,
//     pub program_id: String,
// }

/// A duplicate representation of TransactionStatusMeta with `err` field. Copied from solana-transactions-status crate, but without the status field.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiTransactionStatusMeta {
    pub err: Option<TransactionError>,
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub inner_instructions: OptionSerializer<Vec<UiInnerInstructions>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub log_messages: OptionSerializer<Vec<String>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub pre_token_balances: OptionSerializer<Vec<UiTransactionTokenBalance>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub post_token_balances: OptionSerializer<Vec<UiTransactionTokenBalance>>,
    #[serde(
        default = "OptionSerializer::none",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub rewards: OptionSerializer<Rewards>,
    #[serde(
        default = "OptionSerializer::skip",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub loaded_addresses: OptionSerializer<UiLoadedAddresses>,
    #[serde(
        default = "OptionSerializer::skip",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub return_data: OptionSerializer<UiTransactionReturnData>,
    #[serde(
        default = "OptionSerializer::skip",
        skip_serializing_if = "OptionSerializer::should_skip"
    )]
    pub compute_units_consumed: OptionSerializer<u64>,
}

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum TransactionType {
    UNKNOWN,
    ANY,
    NFT_BID,
    NFT_GLOBAL_BID,
    NFT_GLOBAL_BID_CANCELLED,
    NFT_BID_CANCELLED,
    NFT_LISTING,
    NFT_CANCEL_LISTING,
    NFT_SALE,
    NFT_MINT,
    NFT_AUCTION_CREATED,
    NFT_AUCTION_UPDATED,
    NFT_AUCTION_CANCELLED,
    NFT_PARTICIPATION_REWARD,
    NFT_MINT_REJECTED,
    CREATE_STORE,
    WHITELIST_CREATOR,
    ADD_TO_WHITELIST,
    REMOVE_FROM_WHITELIST,
    AUCTION_MANAGER_CLAIM_BID,
    EMPTY_PAYMENT_ACCOUNT,
    UPDATE_PRIMARY_SALE_METADATA,
    ADD_TOKEN_TO_VAULT,
    ACTIVATE_VAULT,
    INIT_VAULT,
    INIT_BANK,
    INIT_STAKE,
    MERGE_STAKE,
    SPLIT_STAKE,
    SET_BANK_FLAGS,
    SET_VAULT_LOCK,
    UPDATE_VAULT_OWNER,
    UPDATE_BANK_MANAGER,
    RECORD_RARITY_POINTS,
    ADD_RARITIES_TO_BANK,
    INIT_FARM,
    INIT_FARMER,
    REFRESH_FARMER,
    UPDATE_FARM,
    AUTHORIZE_FUNDER,
    DEAUTHORIZE_FUNDER,
    FUND_REWARD,
    CANCEL_REWARD,
    LOCK_REWARD,
    PAYOUT,
    VALIDATE_SAFETY_DEPOSIT_BOX_V2,
    SET_AUTHORITY,
    INIT_AUCTION_MANAGER_V2,
    UPDATE_EXTERNAL_PRICE_ACCOUNT,
    AUCTION_HOUSE_CREATE,
    CLOSE_ESCROW_ACCOUNT,
    WITHDRAW,
    DEPOSIT,
    TRANSFER,
    BURN,
    BURN_NFT,
    PLATFORM_FEE,
    LOAN,
    RESCIND_LOAN,
    OFFER_LOAN,
    REPAY_LOAN,
    TAKE_LOAN,
    FORECLOSE_LOAN,
    ADD_TO_POOL,
    REMOVE_FROM_POOL,
    CLOSE_POSITION,
    UNLABELED,
    CLOSE_ACCOUNT,
    WITHDRAW_GEM,
    DEPOSIT_GEM,
    STAKE_TOKEN,
    UNSTAKE_TOKEN,
    STAKE_SOL,
    UNSTAKE_SOL,
    CLAIM_REWARDS,
    BUY_SUBSCRIPTION,
    SWAP,
    INIT_SWAP,
    CANCEL_SWAP,
    REJECT_SWAP,
    INITIALIZE_ACCOUNT,
    TOKEN_MINT,
    CREATE_APPRAISAL,
    CANDY_MACHINE_WRAP,
    CANDY_MACHINE_UNWRAP,
    CANDY_MACHINE_UPDATE,
    CANDY_MACHINE_ROUTE,
    FRACTIONALIZE,
    DEPOSIT_FRACTIONAL_POOL,
    FUSE,
    CREATE_RAFFLE,
    BUY_TICKETS,
    UPDATE_ITEM,
    LIST_ITEM,
    DELIST_ITEM,
    ADD_ITEM,
    CLOSE_ITEM,
}
impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum TransactionSource {
    FORM_FUNCTION,
    EXCHANGE_ART,
    CANDY_MACHINE_V3,
    CANDY_MACHINE_V2,
    CANDY_MACHINE_V1,
    UNKNOWN,
    SOLANART,
    SOLSEA,
    MAGIC_EDEN,
    HOLAPLEX,
    METAPLEX,
    OPENSEA,
    SOLANA_PROGRAM_LIBRARY,
    ANCHOR,
    PHANTOM,
    SYSTEM_PROGRAM,
    STAKE_PROGRAM,
    COINBASE,
    CORAL_CUBE,
    HEDGE,
    LAUNCH_MY_NFT,
    GEM_BANK,
    GEM_FARM,
    DEGODS,
    BLOCKSMITH_LABS,
    YAWWW,
    ATADIA,
    DIGITAL_EYES,
    HYPERSPACE,
    TENSOR,
    BIFROST,
    JUPITER,
    MERCURIAL_STABLE_SWAP,
    SABER,
    SERUM,
    STEP_FINANCE,
    CROPPER,
    RAYDIUM,
    ALDRIN,
    CREMA,
    LIFINITY,
    CYKURA,
    ORCA,
    MARINADE,
    STEPN,
    SENCHA_EXCHANGE,
    SAROS,
    ENGLISH_AUCTION,
    FOXY,
    HADESWAP,
    FOXY_STAKING,
    FOXY_RAFFLE,
    FOXY_TOKEN_MARKET,
    FOXY_MISSIONS,
    FOXY_MARMALADE,
    FOXY_COINFLIP,
    FOXY_AUCTION,
    CITRUS,
    ZETA,
    ELIXIR,
    ELIXIR_LAUNCHPAD,
    CARDINAL_RENT,
    CARDINAL_STAKING,
    BPF_LOADER,
    BPF_UPGRADEABLE_LOADER,
    SQUADS,
    SHARKY_FI,
    OPEN_CREATOR_PROTOCOL,

    // Mints
    W_SOL,
    DUST,
    SOLI,
    USDC,
    FLWR,
    HDG,
    MEAN,
    UXD,
    SHDW,
    POLIS,
    ATLAS,
    USH,
    TRTLS,
    RUNNER,
    INVICTUS,
}
impl fmt::Display for TransactionSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
