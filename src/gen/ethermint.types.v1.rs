// @generated
/// EthAccount implements the authtypes.AccountI interface and embeds an
/// authtypes.BaseAccount type. It is compatible with the auth AccountKeeper.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthAccount {
    /// base_account is an authtypes.BaseAccount
    #[prost(message, optional, tag="1")]
    pub base_account: ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    /// code_hash is the hash calculated from the code contents
    #[prost(string, tag="2")]
    pub code_hash: ::prost::alloc::string::String,
}
/// ExtensionOptionDynamicFeeTx is an extension option that specifies the maxPrioPrice for cosmos tx
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionOptionDynamicFeeTx {
    /// max_priority_price is the same as `max_priority_fee_per_gas` in eip-1559 spec
    #[prost(string, tag="1")]
    pub max_priority_price: ::prost::alloc::string::String,
}
/// TxResult is the value stored in eth tx indexer
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResult {
    /// height of the blockchain
    #[prost(int64, tag="1")]
    pub height: i64,
    /// tx_index of the cosmos transaction
    #[prost(uint32, tag="2")]
    pub tx_index: u32,
    /// msg_index in a batch transaction
    #[prost(uint32, tag="3")]
    pub msg_index: u32,
    /// eth_tx_index is the index in the list of valid eth tx in the block,
    /// aka. the transaction list returned by eth_getBlock api.
    #[prost(int32, tag="4")]
    pub eth_tx_index: i32,
    /// failed is true if the eth transaction did not go succeed
    #[prost(bool, tag="5")]
    pub failed: bool,
    /// gas_used by the transaction. If it exceeds the block gas limit,
    /// it's set to gas limit, which is what's actually deducted by ante handler.
    #[prost(uint64, tag="6")]
    pub gas_used: u64,
    /// cumulative_gas_used specifies the cumulated amount of gas used for all
    /// processed messages within the current batch transaction.
    #[prost(uint64, tag="7")]
    pub cumulative_gas_used: u64,
}
/// ExtensionOptionsWeb3Tx is an extension option that specifies the typed chain id,
/// the fee payer as well as its signature data.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionOptionsWeb3Tx {
    /// typed_data_chain_id is used only in EIP712 Domain and should match
    /// Ethereum network ID in a Web3 provider (e.g. Metamask).
    #[prost(uint64, tag="1")]
    pub typed_data_chain_id: u64,
    /// fee_payer is an account address for the fee payer. It will be validated
    /// during EIP712 signature checking.
    #[prost(string, tag="2")]
    pub fee_payer: ::prost::alloc::string::String,
    /// fee_payer_sig is a signature data from the fee paying account,
    /// allows to perform fee delegation when using EIP712 Domain.
    #[prost(bytes="bytes", tag="3")]
    pub fee_payer_sig: ::prost::bytes::Bytes,
}
include!("ethermint.types.v1.serde.rs");
// @@protoc_insertion_point(module)