# CosmosPeriodBankPeriodV1beta1PeriodMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> |  | [optional]
**denom_units** | Option<[**Vec<crate::models::CosmosPeriodBankPeriodV1beta1PeriodDenomUnit>**](cosmos.bank.v1beta1.DenomUnit.md)> |  | [optional]
**base** | Option<**String**> | base represents the base denom (should be the DenomUnit with exponent = 0). | [optional]
**display** | Option<**String**> | display indicates the suggested denom that should be displayed in clients. | [optional]
**name** | Option<**String**> | Since: cosmos-sdk 0.43 | [optional]
**symbol** | Option<**String**> | symbol is the token symbol usually shown on exchanges (eg: ATOM). This can be the same as the display.  Since: cosmos-sdk 0.43 | [optional]
**uri** | Option<**String**> | URI to a document (on or off-chain) that contains additional information. Optional.  Since: cosmos-sdk 0.46 | [optional]
**uri_hash** | Option<**String**> | URIHash is a sha256 hash of a document pointed by URI. It's used to verify that the document didn't change. Optional.  Since: cosmos-sdk 0.46 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


