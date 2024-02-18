// @generated
/// GenesisState defines the liquidvesting module's genesis state.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// Params holds parameters for the liquidvesting module.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag="1")]
    pub minimum_liquidation_amount: ::prost::alloc::string::String,
}
/// Denom represents liquid token bonded to some specific vesting schedule
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Denom {
    /// base_denom main identifier for the denom, used to query it from store.
    #[prost(string, tag="1")]
    pub base_denom: ::prost::alloc::string::String,
    /// display_denom identifier used for display name for broad audience
    #[prost(string, tag="2")]
    pub display_denom: ::prost::alloc::string::String,
    /// original_denom which liquid denom derived from
    #[prost(string, tag="3")]
    pub original_denom: ::prost::alloc::string::String,
    /// start date
    #[prost(message, optional, tag="4")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// end_date
    #[prost(message, optional, tag="5")]
    pub end_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// lockup periods
    #[prost(message, repeated, tag="6")]
    pub lockup_periods: ::prost::alloc::vec::Vec<super::super::super::cosmos::vesting::v1beta1::Period>,
}
/// QueryDenomRequest is request fo Denom rpc method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomRequest {
    /// denom is liquidated vesting token
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryDenomResponse is response for Denom rpc method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomResponse {
    /// denom is liquidated vesting token
    #[prost(message, optional, tag="1")]
    pub denom: ::core::option::Option<Denom>,
}
/// QueryDenomsRequest is request for Denoms rpc method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryDenomsResponse is response for Denoms rpc method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomsResponse {
    /// denoms are liquidated vesting tokens
    #[prost(message, repeated, tag="1")]
    pub denoms: ::prost::alloc::vec::Vec<Denom>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgLiquidate represents message to liquidate arbitrary amount of tokens locked in vesting
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidate {
    /// account for liquidation of locked vesting tokens
    #[prost(string, tag="1")]
    pub liquidate_from: ::prost::alloc::string::String,
    /// account to send resulted liquid token
    #[prost(string, tag="2")]
    pub liquidate_to: ::prost::alloc::string::String,
    /// amount of tokens subject for liquidation
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgLiquidateResponse defines the Msg/Liquidate response type
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidateResponse {
}
/// MsgLiquidate represents message to redeem arbitrary amount of liquid vesting tokens
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRedeem {
    #[prost(string, tag="1")]
    pub redeem_from: ::prost::alloc::string::String,
    /// destination address for vesting tokens
    #[prost(string, tag="2")]
    pub redeem_to: ::prost::alloc::string::String,
    /// amount of vesting tokens to redeem from liquidation module
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgRedeemResponse defines the Msg/Redeem response type
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRedeemResponse {
}
include!("haqq.liquidvesting.v1.serde.rs");
include!("haqq.liquidvesting.v1.tonic.rs");
// @@protoc_insertion_point(module)