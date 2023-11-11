// @generated
/// PubKey is an ed25519 public key for handling Tendermint keys in SDK.
/// It's needed for Any serialization and SDK compatibility.
/// It must not be used in a non Tendermint key context because it doesn't implement
/// ADR-28. Nevertheless, you will like to use ed25519 in app user level
/// then you must create a new proto message and follow ADR-28 for Address construction.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    #[prost(bytes="bytes", tag="1")]
    pub key: ::prost::bytes::Bytes,
}
/// Deprecated: PrivKey defines a ed25519 private key.
/// NOTE: ed25519 keys must not be used in SDK apps except in a tendermint validator context.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivKey {
    #[prost(bytes="bytes", tag="1")]
    pub key: ::prost::bytes::Bytes,
}
include!("cosmos.crypto.ed25519.serde.rs");
// @@protoc_insertion_point(module)