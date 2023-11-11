// @generated
/// PubKey defines a type alias for an ecdsa.PublicKey that implements
/// Tendermint's PubKey interface. It represents the 33-byte compressed public
/// key format.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    /// key is the public key in byte form
    #[prost(bytes="bytes", tag="1")]
    pub key: ::prost::bytes::Bytes,
}
/// PrivKey defines a type alias for an ecdsa.PrivateKey that implements
/// Tendermint's PrivateKey interface.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivKey {
    /// key is the private key in byte form
    #[prost(bytes="bytes", tag="1")]
    pub key: ::prost::bytes::Bytes,
}
include!("ethermint.crypto.v1.ethsecp256k1.serde.rs");
// @@protoc_insertion_point(module)