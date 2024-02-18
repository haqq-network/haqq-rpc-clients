// @generated
/// EpochInfo defines the message interface containing the relevant informations
/// about an epoch.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochInfo {
    /// identifier of the epoch
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
    /// start_time of the epoch
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// duration of the epoch
    #[prost(message, optional, tag="3")]
    pub duration: ::core::option::Option<::pbjson_types::Duration>,
    /// current_epoch is the integer identifier of the epoch
    #[prost(int64, tag="4")]
    pub current_epoch: i64,
    /// current_epoch_start_time defines the timestamp of the start of the epoch
    #[prost(message, optional, tag="5")]
    pub current_epoch_start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// epoch_counting_started reflects if the counting for the epoch has started
    #[prost(bool, tag="6")]
    pub epoch_counting_started: bool,
    /// current_epoch_start_height of the epoch
    #[prost(int64, tag="7")]
    pub current_epoch_start_height: i64,
}
/// GenesisState defines the epochs module's genesis state.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// epochs is a slice of EpochInfo that defines the epochs in the genesis state
    #[prost(message, repeated, tag="1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
/// QueryEpochsInfoRequest is the request type for the Query/EpochInfos RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochsInfoRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryEpochsInfoResponse is the response type for the Query/EpochInfos RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochsInfoResponse {
    /// epochs is a slice of all EpochInfos
    #[prost(message, repeated, tag="1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryCurrentEpochRequest is the request type for the Query/EpochInfos RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentEpochRequest {
    /// identifier of the current epoch
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
}
/// QueryCurrentEpochResponse is the response type for the Query/EpochInfos RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentEpochResponse {
    /// current_epoch is the number of the current epoch
    #[prost(int64, tag="1")]
    pub current_epoch: i64,
}
include!("evmos.epochs.v1.serde.rs");
include!("evmos.epochs.v1.tonic.rs");
// @@protoc_insertion_point(module)