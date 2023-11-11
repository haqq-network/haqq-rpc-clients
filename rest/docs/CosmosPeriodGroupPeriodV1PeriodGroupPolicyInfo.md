# CosmosPeriodGroupPeriodV1PeriodGroupPolicyInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | address is the account address of group policy. | [optional]
**group_id** | Option<**String**> | group_id is the unique ID of the group. | [optional]
**admin** | Option<**String**> | admin is the account address of the group admin. | [optional]
**metadata** | Option<**String**> | metadata is any arbitrary metadata to attached to the group policy. | [optional]
**version** | Option<**String**> | version is used to track changes to a group's GroupPolicyInfo structure that would create a different result on a running proposal. | [optional]
**decision_policy** | Option<[**crate::models::GooglePeriodProtobufPeriodAny**](google.protobuf.Any.md)> |  | [optional]
**created_at** | Option<**String**> | created_at is a timestamp specifying when a group policy was created. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


