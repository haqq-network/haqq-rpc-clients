# EthermintPeriodEvmPeriodV1PeriodParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**evm_denom** | Option<**String**> | evm_denom represents the token denomination used to run the EVM state transitions. | [optional]
**enable_create** | Option<**bool**> |  | [optional]
**enable_call** | Option<**bool**> |  | [optional]
**extra_eips** | Option<**Vec<String>**> |  | [optional]
**chain_config** | Option<[**crate::models::EthermintPeriodEvmPeriodV1PeriodChainConfig**](ethermint.evm.v1.ChainConfig.md)> |  | [optional]
**allow_unprotected_txs** | Option<**bool**> | allow_unprotected_txs defines if replay-protected (i.e non EIP155 signed) transactions can be executed on the state machine. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


