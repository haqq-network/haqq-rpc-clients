// @generated
/// EventEthereumTx defines the event for an Ethereum transaction
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventEthereumTx {
    /// amount
    #[prost(string, tag="1")]
    pub amount: ::prost::alloc::string::String,
    /// eth_hash is the Ethereum hash of the transaction
    #[prost(string, tag="2")]
    pub eth_hash: ::prost::alloc::string::String,
    /// index of the transaction in the block
    #[prost(string, tag="3")]
    pub index: ::prost::alloc::string::String,
    /// gas_used is the amount of gas used by the transaction
    #[prost(string, tag="4")]
    pub gas_used: ::prost::alloc::string::String,
    /// hash is the Tendermint hash of the transaction
    #[prost(string, tag="5")]
    pub hash: ::prost::alloc::string::String,
    /// recipient of the transaction
    #[prost(string, tag="6")]
    pub recipient: ::prost::alloc::string::String,
    /// eth_tx_failed contains a VM error should it occur
    #[prost(string, tag="7")]
    pub eth_tx_failed: ::prost::alloc::string::String,
}
/// EventTxLog defines the event for an Ethereum transaction log
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTxLog {
    /// tx_logs is an array of transaction logs
    #[prost(string, repeated, tag="1")]
    pub tx_logs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventMessage
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMessage {
    /// module which emits the event
    #[prost(string, tag="1")]
    pub module: ::prost::alloc::string::String,
    /// sender of the message
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    /// tx_type is the type of the message
    #[prost(string, tag="3")]
    pub tx_type: ::prost::alloc::string::String,
}
/// EventBlockBloom defines an Ethereum block bloom filter event
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBlockBloom {
    /// bloom is the bloom filter of the block
    #[prost(string, tag="1")]
    pub bloom: ::prost::alloc::string::String,
}
/// Params defines the EVM module parameters
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// evm_denom represents the token denomination used to run the EVM state
    /// transitions.
    #[prost(string, tag="1")]
    pub evm_denom: ::prost::alloc::string::String,
    /// enable_create toggles state transitions that use the vm.Create function
    #[prost(bool, tag="2")]
    pub enable_create: bool,
    /// enable_call toggles state transitions that use the vm.Call function
    #[prost(bool, tag="3")]
    pub enable_call: bool,
    /// extra_eips defines the additional EIPs for the vm.Config
    #[prost(int64, repeated, packed="false", tag="4")]
    pub extra_eips: ::prost::alloc::vec::Vec<i64>,
    /// chain_config defines the EVM chain configuration parameters
    #[prost(message, optional, tag="5")]
    pub chain_config: ::core::option::Option<ChainConfig>,
    /// allow_unprotected_txs defines if replay-protected (i.e non EIP155
    /// signed) transactions can be executed on the state machine.
    #[prost(bool, tag="6")]
    pub allow_unprotected_txs: bool,
}
/// ChainConfig defines the Ethereum ChainConfig parameters using *sdk.Int values
/// instead of *big.Int.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainConfig {
    /// homestead_block switch (nil no fork, 0 = already homestead)
    #[prost(string, tag="1")]
    pub homestead_block: ::prost::alloc::string::String,
    /// dao_fork_block corresponds to TheDAO hard-fork switch block (nil no fork)
    #[prost(string, tag="2")]
    pub dao_fork_block: ::prost::alloc::string::String,
    /// dao_fork_support defines whether the nodes supports or opposes the DAO hard-fork
    #[prost(bool, tag="3")]
    pub dao_fork_support: bool,
    /// eip150_block: EIP150 implements the Gas price changes
    /// (<https://github.com/ethereum/EIPs/issues/150>) EIP150 HF block (nil no fork)
    #[prost(string, tag="4")]
    pub eip150_block: ::prost::alloc::string::String,
    /// eip150_hash: EIP150 HF hash (needed for header only clients as only gas pricing changed)
    #[prost(string, tag="5")]
    pub eip150_hash: ::prost::alloc::string::String,
    /// eip155_block: EIP155Block HF block
    #[prost(string, tag="6")]
    pub eip155_block: ::prost::alloc::string::String,
    /// eip158_block: EIP158 HF block
    #[prost(string, tag="7")]
    pub eip158_block: ::prost::alloc::string::String,
    /// byzantium_block: Byzantium switch block (nil no fork, 0 = already on byzantium)
    #[prost(string, tag="8")]
    pub byzantium_block: ::prost::alloc::string::String,
    /// constantinople_block: Constantinople switch block (nil no fork, 0 = already activated)
    #[prost(string, tag="9")]
    pub constantinople_block: ::prost::alloc::string::String,
    /// petersburg_block: Petersburg switch block (nil same as Constantinople)
    #[prost(string, tag="10")]
    pub petersburg_block: ::prost::alloc::string::String,
    /// istanbul_block: Istanbul switch block (nil no fork, 0 = already on istanbul)
    #[prost(string, tag="11")]
    pub istanbul_block: ::prost::alloc::string::String,
    /// muir_glacier_block: Eip-2384 (bomb delay) switch block (nil no fork, 0 = already activated)
    #[prost(string, tag="12")]
    pub muir_glacier_block: ::prost::alloc::string::String,
    /// berlin_block: Berlin switch block (nil = no fork, 0 = already on berlin)
    #[prost(string, tag="13")]
    pub berlin_block: ::prost::alloc::string::String,
    /// london_block: London switch block (nil = no fork, 0 = already on london)
    #[prost(string, tag="17")]
    pub london_block: ::prost::alloc::string::String,
    /// arrow_glacier_block: Eip-4345 (bomb delay) switch block (nil = no fork, 0 = already activated)
    #[prost(string, tag="18")]
    pub arrow_glacier_block: ::prost::alloc::string::String,
    /// gray_glacier_block: EIP-5133 (bomb delay) switch block (nil = no fork, 0 = already activated)
    #[prost(string, tag="20")]
    pub gray_glacier_block: ::prost::alloc::string::String,
    /// merge_netsplit_block: Virtual fork after The Merge to use as a network splitter
    #[prost(string, tag="21")]
    pub merge_netsplit_block: ::prost::alloc::string::String,
    /// shanghai_block switch block (nil = no fork, 0 = already on shanghai)
    #[prost(string, tag="22")]
    pub shanghai_block: ::prost::alloc::string::String,
    /// cancun_block switch block (nil = no fork, 0 = already on cancun)
    #[prost(string, tag="23")]
    pub cancun_block: ::prost::alloc::string::String,
}
/// State represents a single Storage key value pair item.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    /// key is the stored key
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    /// value is the stored value for the given key
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
/// TransactionLogs define the logs generated from a transaction execution
/// with a given hash. It it used for import/export data as transactions are not
/// persisted on blockchain state after an upgrade.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionLogs {
    /// hash of the transaction
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    /// logs is an array of Logs for the given transaction hash
    #[prost(message, repeated, tag="2")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
/// Log represents an protobuf compatible Ethereum Log that defines a contract
/// log event. These events are generated by the LOG opcode and stored/indexed by
/// the node.
///
/// NOTE: address, topics and data are consensus fields. The rest of the fields
/// are derived, i.e. filled in by the nodes, but not secured by consensus.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    /// address of the contract that generated the event
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// topics is a list of topics provided by the contract.
    #[prost(string, repeated, tag="2")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// data which is supplied by the contract, usually ABI-encoded
    #[prost(bytes="bytes", tag="3")]
    pub data: ::prost::bytes::Bytes,
    /// block_number of the block in which the transaction was included
    #[prost(uint64, tag="4")]
    pub block_number: u64,
    /// tx_hash is the transaction hash
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    /// tx_index of the transaction in the block
    #[prost(uint64, tag="6")]
    pub tx_index: u64,
    /// block_hash of the block in which the transaction was included
    #[prost(string, tag="7")]
    pub block_hash: ::prost::alloc::string::String,
    /// index of the log in the block
    #[prost(uint64, tag="8")]
    pub index: u64,
    /// removed is true if this log was reverted due to a chain
    /// reorganisation. You must pay attention to this field if you receive logs
    /// through a filter query.
    #[prost(bool, tag="9")]
    pub removed: bool,
}
/// TxResult stores results of Tx execution.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResult {
    /// contract_address contains the ethereum address of the created contract (if
    /// any). If the state transition is an evm.Call, the contract address will be
    /// empty.
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// bloom represents the bloom filter bytes
    #[prost(bytes="bytes", tag="2")]
    pub bloom: ::prost::bytes::Bytes,
    /// tx_logs contains the transaction hash and the proto-compatible ethereum
    /// logs.
    #[prost(message, optional, tag="3")]
    pub tx_logs: ::core::option::Option<TransactionLogs>,
    /// ret defines the bytes from the execution.
    #[prost(bytes="bytes", tag="4")]
    pub ret: ::prost::bytes::Bytes,
    /// reverted flag is set to true when the call has been reverted
    #[prost(bool, tag="5")]
    pub reverted: bool,
    /// gas_used notes the amount of gas consumed while execution
    #[prost(uint64, tag="6")]
    pub gas_used: u64,
}
/// AccessTuple is the element type of an access list.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTuple {
    /// address is a hex formatted ethereum address
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// storage_keys are hex formatted hashes of the storage keys
    #[prost(string, repeated, tag="2")]
    pub storage_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// TraceConfig holds extra parameters to trace functions.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceConfig {
    /// tracer is a custom javascript tracer
    #[prost(string, tag="1")]
    pub tracer: ::prost::alloc::string::String,
    /// timeout overrides the default timeout of 5 seconds for JavaScript-based tracing
    /// calls
    #[prost(string, tag="2")]
    pub timeout: ::prost::alloc::string::String,
    /// reexec defines the number of blocks the tracer is willing to go back
    #[prost(uint64, tag="3")]
    pub reexec: u64,
    /// disable_stack switches stack capture
    #[prost(bool, tag="5")]
    pub disable_stack: bool,
    /// disable_storage switches storage capture
    #[prost(bool, tag="6")]
    pub disable_storage: bool,
    /// debug can be used to print output during capture end
    #[prost(bool, tag="8")]
    pub debug: bool,
    /// limit defines the maximum length of output, but zero means unlimited
    #[prost(int32, tag="9")]
    pub limit: i32,
    /// overrides can be used to execute a trace using future fork rules
    #[prost(message, optional, tag="10")]
    pub overrides: ::core::option::Option<ChainConfig>,
    /// enable_memory switches memory capture
    #[prost(bool, tag="11")]
    pub enable_memory: bool,
    /// enable_return_data switches the capture of return data
    #[prost(bool, tag="12")]
    pub enable_return_data: bool,
    /// tracer_json_config configures the tracer using a JSON string
    #[prost(string, tag="13")]
    pub tracer_json_config: ::prost::alloc::string::String,
}
/// GenesisState defines the evm module's genesis state.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// accounts is an array containing the ethereum genesis accounts.
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<GenesisAccount>,
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// GenesisAccount defines an account to be initialized in the genesis state.
/// Its main difference between with Geth's GenesisAccount is that it uses a
/// custom storage type and that it doesn't contain the private key field.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisAccount {
    /// address defines an ethereum hex formated address of an account
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// code defines the hex bytes of the account code.
    #[prost(string, tag="2")]
    pub code: ::prost::alloc::string::String,
    /// storage defines the set of state key values for the account.
    #[prost(message, repeated, tag="3")]
    pub storage: ::prost::alloc::vec::Vec<State>,
}
/// MsgEthereumTx encapsulates an Ethereum transaction as an SDK message.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEthereumTx {
    /// data is inner transaction data of the Ethereum transaction
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<::prost_wkt_types::Any>,
    /// size is the encoded storage size of the transaction (DEPRECATED)
    #[prost(double, tag="2")]
    pub size: f64,
    /// hash of the transaction in hex format
    #[prost(string, tag="3")]
    pub hash: ::prost::alloc::string::String,
    /// from is the ethereum signer address in hex format. This address value is checked
    /// against the address derived from the signature (V, R, S) using the
    /// secp256k1 elliptic curve
    #[prost(string, tag="4")]
    pub from: ::prost::alloc::string::String,
}
/// LegacyTx is the transaction data of regular Ethereum transactions.
/// NOTE: All non-protected transactions (i.e non EIP155 signed) will fail if the
/// AllowUnprotectedTxs parameter is disabled.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyTx {
    /// nonce corresponds to the account nonce (transaction sequence).
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    /// gas_price defines the value for each gas unit
    #[prost(string, tag="2")]
    pub gas_price: ::prost::alloc::string::String,
    /// gas defines the gas limit defined for the transaction.
    #[prost(uint64, tag="3")]
    pub gas: u64,
    /// to is the hex formatted address of the recipient
    #[prost(string, tag="4")]
    pub to: ::prost::alloc::string::String,
    /// value defines the unsigned integer value of the transaction amount.
    #[prost(string, tag="5")]
    pub value: ::prost::alloc::string::String,
    /// data is the data payload bytes of the transaction.
    #[prost(bytes="bytes", tag="6")]
    pub data: ::prost::bytes::Bytes,
    /// v defines the signature value
    #[prost(bytes="bytes", tag="7")]
    pub v: ::prost::bytes::Bytes,
    /// r defines the signature value
    #[prost(bytes="bytes", tag="8")]
    pub r: ::prost::bytes::Bytes,
    /// s define the signature value
    #[prost(bytes="bytes", tag="9")]
    pub s: ::prost::bytes::Bytes,
}
/// AccessListTx is the data of EIP-2930 access list transactions.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessListTx {
    /// chain_id of the destination EVM chain
    #[prost(string, tag="1")]
    pub chain_id: ::prost::alloc::string::String,
    /// nonce corresponds to the account nonce (transaction sequence).
    #[prost(uint64, tag="2")]
    pub nonce: u64,
    /// gas_price defines the value for each gas unit
    #[prost(string, tag="3")]
    pub gas_price: ::prost::alloc::string::String,
    /// gas defines the gas limit defined for the transaction.
    #[prost(uint64, tag="4")]
    pub gas: u64,
    /// to is the recipient address in hex format
    #[prost(string, tag="5")]
    pub to: ::prost::alloc::string::String,
    /// value defines the unsigned integer value of the transaction amount.
    #[prost(string, tag="6")]
    pub value: ::prost::alloc::string::String,
    /// data is the data payload bytes of the transaction.
    #[prost(bytes="bytes", tag="7")]
    pub data: ::prost::bytes::Bytes,
    /// accesses is an array of access tuples
    #[prost(message, repeated, tag="8")]
    pub accesses: ::prost::alloc::vec::Vec<AccessTuple>,
    /// v defines the signature value
    #[prost(bytes="bytes", tag="9")]
    pub v: ::prost::bytes::Bytes,
    /// r defines the signature value
    #[prost(bytes="bytes", tag="10")]
    pub r: ::prost::bytes::Bytes,
    /// s define the signature value
    #[prost(bytes="bytes", tag="11")]
    pub s: ::prost::bytes::Bytes,
}
/// DynamicFeeTx is the data of EIP-1559 dinamic fee transactions.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicFeeTx {
    /// chain_id of the destination EVM chain
    #[prost(string, tag="1")]
    pub chain_id: ::prost::alloc::string::String,
    /// nonce corresponds to the account nonce (transaction sequence).
    #[prost(uint64, tag="2")]
    pub nonce: u64,
    /// gas_tip_cap defines the max value for the gas tip
    #[prost(string, tag="3")]
    pub gas_tip_cap: ::prost::alloc::string::String,
    /// gas_fee_cap defines the max value for the gas fee
    #[prost(string, tag="4")]
    pub gas_fee_cap: ::prost::alloc::string::String,
    /// gas defines the gas limit defined for the transaction.
    #[prost(uint64, tag="5")]
    pub gas: u64,
    /// to is the hex formatted address of the recipient
    #[prost(string, tag="6")]
    pub to: ::prost::alloc::string::String,
    /// value defines the the transaction amount.
    #[prost(string, tag="7")]
    pub value: ::prost::alloc::string::String,
    /// data is the data payload bytes of the transaction.
    #[prost(bytes="bytes", tag="8")]
    pub data: ::prost::bytes::Bytes,
    /// accesses is an array of access tuples
    #[prost(message, repeated, tag="9")]
    pub accesses: ::prost::alloc::vec::Vec<AccessTuple>,
    /// v defines the signature value
    #[prost(bytes="bytes", tag="10")]
    pub v: ::prost::bytes::Bytes,
    /// r defines the signature value
    #[prost(bytes="bytes", tag="11")]
    pub r: ::prost::bytes::Bytes,
    /// s define the signature value
    #[prost(bytes="bytes", tag="12")]
    pub s: ::prost::bytes::Bytes,
}
/// ExtensionOptionsEthereumTx is an extension option for ethereum transactions
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionOptionsEthereumTx {
}
/// MsgEthereumTxResponse defines the Msg/EthereumTx response type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEthereumTxResponse {
    /// hash of the ethereum transaction in hex format. This hash differs from the
    /// Tendermint sha256 hash of the transaction bytes. See
    /// <https://github.com/tendermint/tendermint/issues/6539> for reference
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    /// logs contains the transaction hash and the proto-compatible ethereum
    /// logs.
    #[prost(message, repeated, tag="2")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    /// ret is the returned data from evm function (result or data supplied with revert
    /// opcode)
    #[prost(bytes="bytes", tag="3")]
    pub ret: ::prost::bytes::Bytes,
    /// vm_error is the error returned by vm execution
    #[prost(string, tag="4")]
    pub vm_error: ::prost::alloc::string::String,
    /// gas_used specifies how much gas was consumed by the transaction
    #[prost(uint64, tag="5")]
    pub gas_used: u64,
}
/// MsgUpdateParams defines a Msg for updating the x/evm module parameters.
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
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountRequest {
    /// address is the ethereum hex address to query the account for.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryAccountResponse is the response type for the Query/Account RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountResponse {
    /// balance is the balance of the EVM denomination.
    #[prost(string, tag="1")]
    pub balance: ::prost::alloc::string::String,
    /// code_hash is the hex-formatted code bytes from the EOA.
    #[prost(string, tag="2")]
    pub code_hash: ::prost::alloc::string::String,
    /// nonce is the account's sequence number.
    #[prost(uint64, tag="3")]
    pub nonce: u64,
}
/// QueryCosmosAccountRequest is the request type for the Query/CosmosAccount RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCosmosAccountRequest {
    /// address is the ethereum hex address to query the account for.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryCosmosAccountResponse is the response type for the Query/CosmosAccount
/// RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCosmosAccountResponse {
    /// cosmos_address is the cosmos address of the account.
    #[prost(string, tag="1")]
    pub cosmos_address: ::prost::alloc::string::String,
    /// sequence is the account's sequence number.
    #[prost(uint64, tag="2")]
    pub sequence: u64,
    /// account_number is the account number
    #[prost(uint64, tag="3")]
    pub account_number: u64,
}
/// QueryValidatorAccountRequest is the request type for the
/// Query/ValidatorAccount RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorAccountRequest {
    /// cons_address is the validator cons address to query the account for.
    #[prost(string, tag="1")]
    pub cons_address: ::prost::alloc::string::String,
}
/// QueryValidatorAccountResponse is the response type for the
/// Query/ValidatorAccount RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorAccountResponse {
    /// account_address is the cosmos address of the account in bech32 format.
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
    /// sequence is the account's sequence number.
    #[prost(uint64, tag="2")]
    pub sequence: u64,
    /// account_number is the account number
    #[prost(uint64, tag="3")]
    pub account_number: u64,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceRequest {
    /// address is the ethereum hex address to query the balance for.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    /// balance is the balance of the EVM denomination.
    #[prost(string, tag="1")]
    pub balance: ::prost::alloc::string::String,
}
/// QueryStorageRequest is the request type for the Query/Storage RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorageRequest {
    /// address is the ethereum hex address to query the storage state for.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// key defines the key of the storage state
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
/// QueryStorageResponse is the response type for the Query/Storage RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorageResponse {
    /// value defines the storage state value hash associated with the given key.
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
}
/// QueryCodeRequest is the request type for the Query/Code RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeRequest {
    /// address is the ethereum hex address to query the code for.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryCodeResponse is the response type for the Query/Code RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeResponse {
    /// code represents the code bytes from an ethereum address.
    #[prost(bytes="bytes", tag="1")]
    pub code: ::prost::bytes::Bytes,
}
/// QueryTxLogsRequest is the request type for the Query/TxLogs RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTxLogsRequest {
    /// hash is the ethereum transaction hex hash to query the logs for.
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryTxLogsResponse is the response type for the Query/TxLogs RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTxLogsResponse {
    /// logs represents the ethereum logs generated from the given transaction.
    #[prost(message, repeated, tag="1")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryParamsRequest defines the request type for querying x/evm parameters.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse defines the response type for querying x/evm parameters.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params define the evm module parameters.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// EthCallRequest defines EthCall request
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthCallRequest {
    /// args uses the same json format as the json rpc api.
    #[prost(bytes="bytes", tag="1")]
    pub args: ::prost::bytes::Bytes,
    /// gas_cap defines the default gas cap to be used
    #[prost(uint64, tag="2")]
    pub gas_cap: u64,
    /// proposer_address of the requested block in hex format
    #[prost(bytes="bytes", tag="3")]
    pub proposer_address: ::prost::bytes::Bytes,
    /// chain_id is the eip155 chain id parsed from the requested block header
    #[prost(int64, tag="4")]
    pub chain_id: i64,
}
/// EstimateGasResponse defines EstimateGas response
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateGasResponse {
    /// gas returns the estimated gas
    #[prost(uint64, tag="1")]
    pub gas: u64,
}
/// QueryTraceTxRequest defines TraceTx request
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceTxRequest {
    /// msg is the MsgEthereumTx for the requested transaction
    #[prost(message, optional, tag="1")]
    pub msg: ::core::option::Option<MsgEthereumTx>,
    /// trace_config holds extra parameters to trace functions.
    #[prost(message, optional, tag="3")]
    pub trace_config: ::core::option::Option<TraceConfig>,
    /// predecessors is an array of transactions included in the same block
    /// need to be replayed first to get correct context for tracing.
    #[prost(message, repeated, tag="4")]
    pub predecessors: ::prost::alloc::vec::Vec<MsgEthereumTx>,
    /// block_number of requested transaction
    #[prost(int64, tag="5")]
    pub block_number: i64,
    /// block_hash of requested transaction
    #[prost(string, tag="6")]
    pub block_hash: ::prost::alloc::string::String,
    /// block_time of requested transaction
    #[prost(message, optional, tag="7")]
    pub block_time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// proposer_address is the proposer of the requested block
    #[prost(bytes="bytes", tag="8")]
    pub proposer_address: ::prost::bytes::Bytes,
    /// chain_id is the the eip155 chain id parsed from the requested block header
    #[prost(int64, tag="9")]
    pub chain_id: i64,
}
/// QueryTraceTxResponse defines TraceTx response
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceTxResponse {
    /// data is the response serialized in bytes
    #[prost(bytes="bytes", tag="1")]
    pub data: ::prost::bytes::Bytes,
}
/// QueryTraceBlockRequest defines TraceTx request
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceBlockRequest {
    /// txs is an array of messages in the block
    #[prost(message, repeated, tag="1")]
    pub txs: ::prost::alloc::vec::Vec<MsgEthereumTx>,
    /// trace_config holds extra parameters to trace functions.
    #[prost(message, optional, tag="3")]
    pub trace_config: ::core::option::Option<TraceConfig>,
    /// block_number of the traced block
    #[prost(int64, tag="5")]
    pub block_number: i64,
    /// block_hash (hex) of the traced block
    #[prost(string, tag="6")]
    pub block_hash: ::prost::alloc::string::String,
    /// block_time of the traced block
    #[prost(message, optional, tag="7")]
    pub block_time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// proposer_address is the address of the requested block
    #[prost(bytes="bytes", tag="8")]
    pub proposer_address: ::prost::bytes::Bytes,
    /// chain_id is the eip155 chain id parsed from the requested block header
    #[prost(int64, tag="9")]
    pub chain_id: i64,
}
/// QueryTraceBlockResponse defines TraceBlock response
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceBlockResponse {
    /// data is the response serialized in bytes
    #[prost(bytes="bytes", tag="1")]
    pub data: ::prost::bytes::Bytes,
}
/// QueryBaseFeeRequest defines the request type for querying the EIP1559 base
/// fee.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseFeeRequest {
}
/// QueryBaseFeeResponse returns the EIP1559 base fee.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseFeeResponse {
    /// base_fee is the EIP1559 base fee
    #[prost(string, tag="1")]
    pub base_fee: ::prost::alloc::string::String,
}
include!("ethermint.evm.v1.serde.rs");
include!("ethermint.evm.v1.tonic.rs");
// @@protoc_insertion_point(module)