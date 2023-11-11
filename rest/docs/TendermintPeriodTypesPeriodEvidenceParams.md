# TendermintPeriodTypesPeriodEvidenceParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_age_num_blocks** | Option<**String**> | Max age of evidence, in blocks.  The basic formula for calculating this is: MaxAgeDuration / {average block time}. | [optional]
**max_age_duration** | Option<**String**> | Max age of evidence, in time.  It should correspond with an app's \"unbonding period\" or other similar mechanism for handling [Nothing-At-Stake attacks](https://github.com/ethereum/wiki/wiki/Proof-of-Stake-FAQ#what-is-the-nothing-at-stake-problem-and-how-can-it-be-fixed). | [optional]
**max_bytes** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


