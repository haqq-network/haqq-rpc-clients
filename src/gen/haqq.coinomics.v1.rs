// @generated
/// GenesisState defines the inflation module's genesis state.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// prev block block ts
    #[prost(string, tag="2")]
    pub prev_block_ts: ::prost::alloc::string::String,
    /// max supply
    #[prost(message, optional, tag="3")]
    pub max_supply: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Params holds parameters for the coinomics module.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// type of coin to mint
    #[prost(string, tag="1")]
    pub mint_denom: ::prost::alloc::string::String,
    /// parameter to enable coinmoics
    #[prost(bool, tag="2")]
    pub enable_coinomics: bool,
    /// current staking reward coefficient
    #[prost(string, tag="3")]
    pub reward_coefficient: ::prost::alloc::string::String,
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMaxSupplyRequest {
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMaxSupplyResponse {
    #[prost(message, optional, tag="1")]
    pub max_supply: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardCoefficientRequest {
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardCoefficientResponse {
    /// rate by which the total supply increases within one era
    #[prost(string, tag="1")]
    pub reward_coefficient: ::prost::alloc::string::String,
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
include!("haqq.coinomics.v1.serde.rs");
include!("haqq.coinomics.v1.tonic.rs");
// @@protoc_insertion_point(module)