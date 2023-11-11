// @generated
/// TokenPair defines an instance that records a pairing consisting of a native
///   Cosmos Coin and an ERC20 token address.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenPair {
    /// erc20_address is the hex address of ERC20 contract token
    #[prost(string, tag="1")]
    pub erc20_address: ::prost::alloc::string::String,
    /// denom defines the cosmos base denomination to be mapped to
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// enabled defines the token mapping enable status
    #[prost(bool, tag="3")]
    pub enabled: bool,
    /// contract_owner is the an ENUM specifying the type of ERC20 owner (0 invalid, 1 ModuleAccount, 2 external address)
    #[prost(enumeration="Owner", tag="4")]
    pub contract_owner: i32,
}
/// RegisterCoinProposal is a gov Content type to register a token pair for a
/// native Cosmos coin.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterCoinProposal {
    /// title of the proposal
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// description of the proposal
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// metadata slice of the native Cosmos coins
    #[prost(message, repeated, tag="3")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
/// RegisterERC20Proposal is a gov Content type to register a token pair for an
/// ERC20 token
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterErc20Proposal {
    /// title of the proposal
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// description of the proposal
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// erc20addresses is a slice of  ERC20 token contract addresses
    #[prost(string, repeated, tag="3")]
    pub erc20addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ToggleTokenConversionProposal is a gov Content type to toggle the conversion
/// of a token pair.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleTokenConversionProposal {
    /// title of the proposal
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// description of the proposal
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// token identifier can be either the hex contract address of the ERC20 or the
    /// Cosmos base denomination
    #[prost(string, tag="3")]
    pub token: ::prost::alloc::string::String,
}
/// ProposalMetadata is used to parse a slice of denom metadata and generate
/// the RegisterCoinProposal content.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalMetadata {
    /// metadata slice of the native Cosmos coins
    #[prost(message, repeated, tag="1")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
/// Owner enumerates the ownership of a ERC20 contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Owner {
    /// OWNER_UNSPECIFIED defines an invalid/undefined owner.
    Unspecified = 0,
    /// OWNER_MODULE - erc20 is owned by the erc20 module account.
    Module = 1,
    /// OWNER_EXTERNAL - erc20 is owned by an external account.
    External = 2,
}
impl Owner {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Owner::Unspecified => "OWNER_UNSPECIFIED",
            Owner::Module => "OWNER_MODULE",
            Owner::External => "OWNER_EXTERNAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OWNER_UNSPECIFIED" => Some(Self::Unspecified),
            "OWNER_MODULE" => Some(Self::Module),
            "OWNER_EXTERNAL" => Some(Self::External),
            _ => None,
        }
    }
}
/// EventRegisterPair is an event emitted when a coin is registered.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRegisterPair {
    /// denom is the coin's denomination.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// erc20_address is the ERC20 contract address.
    #[prost(string, tag="2")]
    pub erc20_address: ::prost::alloc::string::String,
}
/// EventToggleTokenConversion is an event emitted when a coin's token conversion is toggled.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventToggleTokenConversion {
    /// denom is the coin's denomination.
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// erc20_address is the ERC20 contract address.
    #[prost(string, tag="2")]
    pub erc20_address: ::prost::alloc::string::String,
}
/// EventConvertCoin is an event emitted when a coin is converted.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventConvertCoin {
    /// sender is the sender's address.
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// receiver is the receiver's address.
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    /// amount is the amount of coins to be converted.
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// denom is the coin's denomination.
    #[prost(string, tag="4")]
    pub denom: ::prost::alloc::string::String,
    /// erc20_address is the ERC20 contract address.
    #[prost(string, tag="5")]
    pub erc20_address: ::prost::alloc::string::String,
}
/// EventConvertERC20 is an event emitted when an ERC20 is converted.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventConvertErc20 {
    /// sender is the sender's address.
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// receiver is the receiver's address.
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    /// amount is the amount of coins to be converted.
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// denom is the coin's denomination.
    #[prost(string, tag="4")]
    pub denom: ::prost::alloc::string::String,
    /// contract_address of an ERC20 token contract, that is registered in a token pair
    #[prost(string, tag="5")]
    pub contract_address: ::prost::alloc::string::String,
}
/// GenesisState defines the module's genesis state.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params are the erc20 module parameters at genesis
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// token_pairs is a slice of the registered token pairs at genesis
    #[prost(message, repeated, tag="2")]
    pub token_pairs: ::prost::alloc::vec::Vec<TokenPair>,
}
/// Params defines the erc20 module params
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// enable_erc20 is the parameter to enable the conversion of Cosmos coins <--> ERC20 tokens.
    #[prost(bool, tag="1")]
    pub enable_erc20: bool,
    /// enable_evm_hook is the parameter to enable the EVM hook that converts an ERC20 token to a Cosmos
    /// Coin by transferring the Tokens through a MsgEthereumTx to the ModuleAddress Ethereum address.
    #[prost(bool, tag="2")]
    pub enable_evm_hook: bool,
}
/// QueryTokenPairsRequest is the request type for the Query/TokenPairs RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenPairsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryTokenPairsResponse is the response type for the Query/TokenPairs RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenPairsResponse {
    /// token_pairs is a slice of registered token pairs for the erc20 module
    #[prost(message, repeated, tag="1")]
    pub token_pairs: ::prost::alloc::vec::Vec<TokenPair>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryTokenPairRequest is the request type for the Query/TokenPair RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenPairRequest {
    /// token identifier can be either the hex contract address of the ERC20 or the
    /// Cosmos base denomination
    #[prost(string, tag="1")]
    pub token: ::prost::alloc::string::String,
}
/// QueryTokenPairResponse is the response type for the Query/TokenPair RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenPairResponse {
    /// token_pairs returns the info about a registered token pair for the erc20 module
    #[prost(message, optional, tag="1")]
    pub token_pair: ::core::option::Option<TokenPair>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params are the erc20 module parameters
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgConvertCoin defines a Msg to convert a native Cosmos coin to a ERC20 token
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConvertCoin {
    /// coin is a Cosmos coin whose denomination is registered in a token pair. The coin
    /// amount defines the amount of coins to convert.
    #[prost(message, optional, tag="1")]
    pub coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// receiver is the hex address to receive ERC20 token
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    /// sender is the cosmos bech32 address from the owner of the given Cosmos coins
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgConvertCoinResponse returns no fields
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConvertCoinResponse {
}
/// MsgConvertERC20 defines a Msg to convert a ERC20 token to a native Cosmos
/// coin.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConvertErc20 {
    /// contract_address of an ERC20 token contract, that is registered in a token pair
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// amount of ERC20 tokens to convert
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
    /// receiver is the bech32 address to receive native Cosmos coins
    #[prost(string, tag="3")]
    pub receiver: ::prost::alloc::string::String,
    /// sender is the hex address from the owner of the given ERC20 tokens
    #[prost(string, tag="4")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgConvertERC20Response returns no fields
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConvertErc20Response {
}
/// MsgUpdateParams is the Msg/UpdateParams request type for Erc20 parameters.
/// Since: cosmos-sdk 0.47
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/evm parameters to update.
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
/// Since: cosmos-sdk 0.47
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
include!("evmos.erc20.v1.serde.rs");
include!("evmos.erc20.v1.tonic.rs");
// @@protoc_insertion_point(module)