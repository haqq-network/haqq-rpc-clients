# CosmosPeriodStakingPeriodV1beta1PeriodValidator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operator_address** | Option<**String**> | operator_address defines the address of the validator's operator; bech encoded in JSON. | [optional]
**consensus_pubkey** | Option<[**crate::models::GooglePeriodProtobufPeriodAny**](google.protobuf.Any.md)> |  | [optional]
**jailed** | Option<**bool**> | jailed defined whether the validator has been jailed from bonded status or not. | [optional]
**status** | Option<[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodBondStatus**](cosmos.staking.v1beta1.BondStatus.md)> |  | [optional]
**tokens** | Option<**String**> | tokens define the delegated tokens (incl. self-delegation). | [optional]
**delegator_shares** | Option<**String**> | delegator_shares defines total shares issued to a validator's delegators. | [optional]
**description** | Option<[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodDescription**](cosmos.staking.v1beta1.Description.md)> |  | [optional]
**unbonding_height** | Option<**String**> | unbonding_height defines, if unbonding, the height at which this validator has begun unbonding. | [optional]
**unbonding_time** | Option<**String**> | unbonding_time defines, if unbonding, the min time for the validator to complete unbonding. | [optional]
**commission** | Option<[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodCommission**](cosmos.staking.v1beta1.Commission.md)> |  | [optional]
**min_self_delegation** | Option<**String**> | min_self_delegation is the validator's self declared minimum self delegation.  Since: cosmos-sdk 0.46 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


