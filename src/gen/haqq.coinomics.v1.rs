// @generated
/// GenesisState defines the inflation module's genesis state.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// current inflation rate
    #[prost(string, tag="2")]
    pub inflation: ::prost::alloc::string::String,
    /// current era number
    #[prost(uint64, tag="3")]
    pub era: u64,
    /// era started block number
    #[prost(uint64, tag="4")]
    pub era_started_at_block: u64,
    /// target mint for current era
    #[prost(message, optional, tag="5")]
    pub era_target_mint: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// closing supply for current era
    #[prost(message, optional, tag="6")]
    pub era_closing_supply: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// max supply
    #[prost(message, optional, tag="7")]
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
    /// number of blocks per era
    #[prost(uint64, tag="2")]
    pub blocks_per_era: u64,
    /// parameter to enable coinmoics
    #[prost(bool, tag="3")]
    pub enable_coinomics: bool,
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEraRequest {
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEraResponse {
    #[prost(uint64, tag="1")]
    pub era: u64,
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEraClosingSupplyRequest {
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEraClosingSupplyResponse {
    #[prost(message, optional, tag="1")]
    pub era_closing_supply: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
pub struct QueryInflationRateRequest {
}
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInflationRateResponse {
    /// rate by which the total supply increases within one era
    #[prost(string, tag="1")]
    pub inflation_rate: ::prost::alloc::string::String,
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