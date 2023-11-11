# TendermintPeriodAbciPeriodResponseCheckTx

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**i64**> |  | [optional]
**data** | Option<**String**> |  | [optional]
**log** | Option<**String**> |  | [optional]
**info** | Option<**String**> |  | [optional]
**gas_wanted** | Option<**String**> |  | [optional]
**gas_used** | Option<**String**> |  | [optional]
**events** | Option<[**Vec<crate::models::TendermintPeriodAbciPeriodEvent>**](tendermint.abci.Event.md)> |  | [optional]
**codespace** | Option<**String**> |  | [optional]
**sender** | Option<**String**> |  | [optional]
**priority** | Option<**String**> |  | [optional]
**mempool_error** | Option<**String**> | mempool_error is set by CometBFT. ABCI applictions creating a ResponseCheckTX should not set mempool_error. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


