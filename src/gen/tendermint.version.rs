// @generated
/// App includes the protocol and software version for the application.
/// This information is included in ResponseInfo. The App.Protocol can be
/// updated in ResponseEndBlock.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct App {
    #[prost(uint64, tag="1")]
    pub protocol: u64,
    #[prost(string, tag="2")]
    pub software: ::prost::alloc::string::String,
}
/// Consensus captures the consensus rules for processing a block in the blockchain,
/// including all blockchain data structures and the rules of the application's
/// state transition machine.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Consensus {
    #[prost(uint64, tag="1")]
    pub block: u64,
    #[prost(uint64, tag="2")]
    pub app: u64,
}
include!("tendermint.version.serde.rs");
// @@protoc_insertion_point(module)