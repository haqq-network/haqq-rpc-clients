// @generated
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BitArray {
    #[prost(int64, tag="1")]
    pub bits: i64,
    #[prost(uint64, repeated, tag="2")]
    pub elems: ::prost::alloc::vec::Vec<u64>,
}
include!("tendermint.libs.bits.serde.rs");
// @@protoc_insertion_point(module)