// @generated
/// GenesisState defines the raw genesis transaction in JSON.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// gen_txs defines the genesis transactions.
    #[prost(bytes="bytes", repeated, tag="1")]
    pub gen_txs: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
}
include!("cosmos.genutil.v1beta1.serde.rs");
// @@protoc_insertion_point(module)