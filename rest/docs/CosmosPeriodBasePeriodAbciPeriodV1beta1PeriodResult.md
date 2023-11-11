# CosmosPeriodBasePeriodAbciPeriodV1beta1PeriodResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<**String**> | Data is any data returned from message or handler execution. It MUST be length prefixed in order to separate data from multiple message executions. Deprecated. This field is still populated, but prefer msg_response instead because it also contains the Msg response typeURL. | [optional]
**log** | Option<**String**> | Log contains the log information from message or handler execution. | [optional]
**events** | Option<[**Vec<crate::models::TendermintPeriodAbciPeriodEvent>**](tendermint.abci.Event.md)> | Events contains a slice of Event objects that were emitted during message or handler execution. | [optional]
**msg_responses** | Option<[**Vec<crate::models::GooglePeriodProtobufPeriodAny>**](google.protobuf.Any.md)> | msg_responses contains the Msg handler responses type packed in Anys.  Since: cosmos-sdk 0.46 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


