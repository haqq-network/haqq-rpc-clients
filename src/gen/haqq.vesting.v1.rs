// @generated
/// EventCreateClawbackVestingAccount defines the event type
/// for creating a clawback vesting account
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateClawbackVestingAccount {
    /// sender is the address of the sender
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// coins to be vested
    #[prost(string, tag="2")]
    pub coins: ::prost::alloc::string::String,
    /// start_time is the time when the coins start to vest
    #[prost(string, tag="3")]
    pub start_time: ::prost::alloc::string::String,
    /// merge
    #[prost(string, tag="4")]
    pub merge: ::prost::alloc::string::String,
    /// account address of recipient
    #[prost(string, tag="5")]
    pub account: ::prost::alloc::string::String,
}
/// EventClawback defines the event type for clawback
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventClawback {
    /// funder is the address of the funder
    #[prost(string, tag="1")]
    pub funder: ::prost::alloc::string::String,
    /// account is the address of the account
    #[prost(string, tag="2")]
    pub account: ::prost::alloc::string::String,
    /// destination is the address of the destination
    #[prost(string, tag="3")]
    pub destination: ::prost::alloc::string::String,
}
/// EventUpdateVestingFunder defines the event type for updating the vesting
/// funder
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateVestingFunder {
    /// funder is the address of the funder
    #[prost(string, tag="1")]
    pub funder: ::prost::alloc::string::String,
    /// account is the address of the account
    #[prost(string, tag="2")]
    pub account: ::prost::alloc::string::String,
    /// new_funder is the address of the new funder
    #[prost(string, tag="3")]
    pub new_funder: ::prost::alloc::string::String,
}
/// QueryBalancesRequest is the request type for the Query/Balances RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalancesRequest {
    /// address of the clawback vesting account
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryBalancesResponse is the response type for the Query/Balances RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalancesResponse {
    /// locked defines the current amount of locked tokens
    #[prost(message, repeated, tag="1")]
    pub locked: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// unvested defines the current amount of unvested tokens
    #[prost(message, repeated, tag="2")]
    pub unvested: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// vested defines the current amount of vested tokens
    #[prost(message, repeated, tag="3")]
    pub vested: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryTotalLockedRequest is the request type for the Query/TotalLocked RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalLockedRequest {
}
/// QueryTotalLockedResponse is the response type for the Query/TotalLocked RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalLockedResponse {
    /// locked defines the current amount of locked tokens
    #[prost(message, repeated, tag="1")]
    pub locked: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// unvested defines the current amount of unvested tokens
    #[prost(message, repeated, tag="2")]
    pub unvested: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// vested defines the current amount of vested tokens
    #[prost(message, repeated, tag="3")]
    pub vested: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgCreateClawbackVestingAccount defines a message that enables creating a
/// ClawbackVestingAccount.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClawbackVestingAccount {
    /// from_address specifies the account to provide the funds and sign the
    /// clawback request
    #[prost(string, tag="1")]
    pub from_address: ::prost::alloc::string::String,
    /// to_address specifies the account to receive the funds
    #[prost(string, tag="2")]
    pub to_address: ::prost::alloc::string::String,
    /// start_time defines the time at which the vesting period begins
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// lockup_periods defines the unlocking schedule relative to the start_time
    #[prost(message, repeated, tag="4")]
    pub lockup_periods: ::prost::alloc::vec::Vec<super::super::super::cosmos::vesting::v1beta1::Period>,
    /// vesting_periods defines the vesting schedule relative to the start_time
    #[prost(message, repeated, tag="5")]
    pub vesting_periods: ::prost::alloc::vec::Vec<super::super::super::cosmos::vesting::v1beta1::Period>,
    /// merge specifies a the creation mechanism for existing
    /// ClawbackVestingAccounts. If true, merge this new grant into an existing
    /// ClawbackVestingAccount, or create it if it does not exist. If false,
    /// creates a new account. New grants to an existing account must be from the
    /// same from_address.
    #[prost(bool, tag="6")]
    pub merge: bool,
}
/// MsgCreateClawbackVestingAccountResponse defines the
/// MsgCreateClawbackVestingAccount response type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClawbackVestingAccountResponse {
}
/// MsgClawback defines a message that removes unvested tokens from a
/// ClawbackVestingAccount.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClawback {
    /// funder_address is the address which funded the account
    #[prost(string, tag="1")]
    pub funder_address: ::prost::alloc::string::String,
    /// account_address is the address of the ClawbackVestingAccount to claw back
    /// from.
    #[prost(string, tag="2")]
    pub account_address: ::prost::alloc::string::String,
    /// dest_address specifies where the clawed-back tokens should be transferred
    /// to. If empty, the tokens will be transferred back to the original funder of
    /// the account.
    #[prost(string, tag="3")]
    pub dest_address: ::prost::alloc::string::String,
}
/// MsgClawbackResponse defines the MsgClawback response type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClawbackResponse {
}
/// MsgUpdateVestingFunder defines a message that updates the funder account of a
/// ClawbackVestingAccount.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateVestingFunder {
    /// funder_address is the current funder address of the ClawbackVestingAccount
    #[prost(string, tag="1")]
    pub funder_address: ::prost::alloc::string::String,
    /// new_funder_address is the new address to replace the existing
    /// funder_address
    #[prost(string, tag="2")]
    pub new_funder_address: ::prost::alloc::string::String,
    /// vesting_address is the address of the ClawbackVestingAccount being updated
    #[prost(string, tag="3")]
    pub vesting_address: ::prost::alloc::string::String,
}
/// MsgUpdateVestingFunderResponse defines the MsgUpdateVestingFunder response
/// type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateVestingFunderResponse {
}
/// MsgConvertVestingAccount defines a message that enables converting a vesting
/// account to a eth account
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConvertVestingAccount {
    /// vesting_address is the address of the vesting account to convert
    #[prost(string, tag="1")]
    pub vesting_address: ::prost::alloc::string::String,
}
/// MsgConvertVestingAccountResponse defines the MsgConvertVestingAccount
/// response type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConvertVestingAccountResponse {
}
/// MsgConvertIntoVestingAccount defines a message that enables converting a eth
/// account to a vesting account
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConvertIntoVestingAccount {
    /// from_address specifies the account to provide the funds and sign the
    /// clawback request
    #[prost(string, tag="1")]
    pub from_address: ::prost::alloc::string::String,
    /// to_address is the account to be converted into clawback vesting account
    #[prost(string, tag="2")]
    pub to_address: ::prost::alloc::string::String,
    /// start_time defines the time at which the vesting period begins
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// lockup_periods defines the unlocking schedule relative to the start_time
    #[prost(message, repeated, tag="4")]
    pub lockup_periods: ::prost::alloc::vec::Vec<super::super::super::cosmos::vesting::v1beta1::Period>,
    /// vesting_periods defines the vesting schedule relative to the start_time
    #[prost(message, repeated, tag="5")]
    pub vesting_periods: ::prost::alloc::vec::Vec<super::super::super::cosmos::vesting::v1beta1::Period>,
    /// merge specifies a the conversion mechanism for existing
    /// ClawbackVestingAccounts. If true, merge this new grant into an existing
    /// ClawbackVestingAccount, or create it if it does not exist. If false,
    /// creates a new account. New grants to an existing account must be from the
    /// same from_address.
    #[prost(bool, tag="6")]
    pub merge: bool,
    /// stake specifies a the post-creation flow. If true, delegate the total
    /// amount to a specified validator. If false, do nothing.
    #[prost(bool, tag="7")]
    pub stake: bool,
    /// validator_address specifies the validator to delegate tokens to.
    #[prost(string, tag="8")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MsgConvertIntoVestingAccountResponse defines the MsgConvertIntoVestingAccount
/// response type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConvertIntoVestingAccountResponse {
}
/// ClawbackVestingAccount implements the VestingAccount interface. It provides
/// an account that can hold contributions subject to "lockup" (like a
/// PeriodicVestingAccount), or vesting which is subject to clawback
/// of unvested tokens, or a combination (tokens vest, but are still locked).
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClawbackVestingAccount {
    /// base_vesting_account implements the VestingAccount interface. It contains
    /// all the necessary fields needed for any vesting account implementation
    #[prost(message, optional, tag="1")]
    pub base_vesting_account: ::core::option::Option<super::super::super::cosmos::vesting::v1beta1::BaseVestingAccount>,
    /// funder_address specifies the account which can perform clawback
    #[prost(string, tag="2")]
    pub funder_address: ::prost::alloc::string::String,
    /// start_time defines the time at which the vesting period begins
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// lockup_periods defines the unlocking schedule relative to the start_time
    #[prost(message, repeated, tag="4")]
    pub lockup_periods: ::prost::alloc::vec::Vec<super::super::super::cosmos::vesting::v1beta1::Period>,
    /// vesting_periods defines the vesting schedule relative to the start_time
    #[prost(message, repeated, tag="5")]
    pub vesting_periods: ::prost::alloc::vec::Vec<super::super::super::cosmos::vesting::v1beta1::Period>,
    /// code_hash is the hash calculated from the code contents
    #[prost(string, tag="6")]
    pub code_hash: ::prost::alloc::string::String,
}
include!("haqq.vesting.v1.serde.rs");
include!("haqq.vesting.v1.tonic.rs");
// @@protoc_insertion_point(module)