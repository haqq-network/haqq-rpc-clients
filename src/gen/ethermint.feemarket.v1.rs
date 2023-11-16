// @generated
/// EventFeeMarket is the event type for the fee market module
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFeeMarket {
    /// base_fee for EIP-1559 blocks
    #[prost(string, tag="1")]
    pub base_fee: ::prost::alloc::string::String,
}
/// EventBlockGas defines an Ethereum block gas event
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBlockGas {
    /// height of the block
    #[prost(string, tag="1")]
    pub height: ::prost::alloc::string::String,
    /// amount of gas wanted by the block
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
/// Params defines the EVM module parameters
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// no_base_fee forces the EIP-1559 base fee to 0 (needed for 0 price calls)
    #[prost(bool, tag="1")]
    pub no_base_fee: bool,
    /// base_fee_change_denominator bounds the amount the base fee can change
    /// between blocks.
    #[prost(uint32, tag="2")]
    pub base_fee_change_denominator: u32,
    /// elasticity_multiplier bounds the maximum gas limit an EIP-1559 block may
    /// have.
    #[prost(uint32, tag="3")]
    pub elasticity_multiplier: u32,
    /// enable_height defines at which block height the base fee calculation is
    /// enabled.
    #[prost(int64, tag="5")]
    pub enable_height: i64,
    /// base_fee for EIP-1559 blocks.
    #[prost(string, tag="6")]
    pub base_fee: ::prost::alloc::string::String,
    /// min_gas_price defines the minimum gas price value for cosmos and eth
    /// transactions
    #[prost(string, tag="7")]
    pub min_gas_price: ::prost::alloc::string::String,
    /// min_gas_multiplier bounds the minimum gas used to be charged
    /// to senders based on gas limit
    #[prost(string, tag="8")]
    pub min_gas_multiplier: ::prost::alloc::string::String,
}
/// GenesisState defines the feemarket module's genesis state.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the feemarket module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// block_gas is the amount of gas wanted on the last block before the upgrade.
    /// Zero by default.
    #[prost(uint64, tag="3")]
    pub block_gas: u64,
}
/// QueryParamsRequest defines the request type for querying x/evm parameters.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse defines the response type for querying x/evm parameters.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params define the evm module parameters.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryBaseFeeRequest defines the request type for querying the EIP1559 base
/// fee.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseFeeRequest {
}
/// QueryBaseFeeResponse returns the EIP1559 base fee.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseFeeResponse {
    /// base_fee is the EIP1559 base fee
    #[prost(string, tag="1")]
    pub base_fee: ::prost::alloc::string::String,
}
/// QueryBlockGasRequest defines the request type for querying the EIP1559 base
/// fee.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockGasRequest {
}
/// QueryBlockGasResponse returns block gas used for a given height.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockGasResponse {
    /// gas is the returned block gas
    #[prost(int64, tag="1")]
    pub gas: i64,
}
/// MsgUpdateParams defines a Msg for updating the x/feemarket module parameters.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/feemarket parameters to update.
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
include!("ethermint.feemarket.v1.serde.rs");
include!("ethermint.feemarket.v1.tonic.rs");
// @@protoc_insertion_point(module)