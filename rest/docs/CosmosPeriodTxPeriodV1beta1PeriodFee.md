# CosmosPeriodTxPeriodV1beta1PeriodFee

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<[**Vec<crate::models::CosmosPeriodBasePeriodV1beta1PeriodCoin>**](cosmos.base.v1beta1.Coin.md)> |  | [optional]
**gas_limit** | Option<**String**> |  | [optional]
**payer** | Option<**String**> | if unset, the first signer is responsible for paying the fees. If set, the specified account must pay the fees. the payer must be a tx signer (and thus have signed this field in AuthInfo). setting this field does *not* change the ordering of required signers for the transaction. | [optional]
**granter** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

