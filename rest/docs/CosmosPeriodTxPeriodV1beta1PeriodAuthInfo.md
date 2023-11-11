# CosmosPeriodTxPeriodV1beta1PeriodAuthInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signer_infos** | Option<[**Vec<crate::models::CosmosPeriodTxPeriodV1beta1PeriodSignerInfo>**](cosmos.tx.v1beta1.SignerInfo.md)> | signer_infos defines the signing modes for the required signers. The number and order of elements must match the required signers from TxBody's messages. The first element is the primary signer and the one which pays the fee. | [optional]
**fee** | Option<[**crate::models::CosmosPeriodTxPeriodV1beta1PeriodFee**](cosmos.tx.v1beta1.Fee.md)> |  | [optional]
**tip** | Option<[**crate::models::CosmosPeriodTxPeriodV1beta1PeriodTip**](cosmos.tx.v1beta1.Tip.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


