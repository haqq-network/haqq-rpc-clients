// @generated
/// PubKey defines a secp256k1 public key
/// Key is the compressed form of the pubkey. The first byte depends is a 0x02 byte
/// if the y-coordinate is the lexicographically largest of the two associated with
/// the x-coordinate. Otherwise the first byte is a 0x03.
/// This prefix is followed with the x-coordinate.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    #[prost(bytes="bytes", tag="1")]
    pub key: ::prost::bytes::Bytes,
}
/// PrivKey defines a secp256k1 private key.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivKey {
    #[prost(bytes="bytes", tag="1")]
    pub key: ::prost::bytes::Bytes,
}
include!("cosmos.crypto.secp256k1.serde.rs");
// @@protoc_insertion_point(module)