# CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodHeader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | Option<[**crate::models::TendermintPeriodVersionPeriodConsensus**](tendermint.version.Consensus.md)> |  | [optional]
**chain_id** | Option<**String**> |  | [optional]
**height** | Option<**String**> |  | [optional]
**time** | Option<**String**> |  | [optional]
**last_block_id** | Option<[**crate::models::TendermintPeriodTypesPeriodBlockId**](tendermint.types.BlockID.md)> |  | [optional]
**last_commit_hash** | Option<**String**> | commit from validators from the last block | [optional]
**data_hash** | Option<**String**> |  | [optional]
**validators_hash** | Option<**String**> | validators for the current block | [optional]
**next_validators_hash** | Option<**String**> |  | [optional]
**consensus_hash** | Option<**String**> |  | [optional]
**app_hash** | Option<**String**> |  | [optional]
**last_results_hash** | Option<**String**> |  | [optional]
**evidence_hash** | Option<**String**> | evidence included in the block | [optional]
**proposer_address** | Option<**String**> | proposer_address is the original block proposer address, formatted as a Bech32 string. In Tendermint, this type is `bytes`, but in the SDK, we convert it to a Bech32 string for better UX.  original proposer of the block | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


