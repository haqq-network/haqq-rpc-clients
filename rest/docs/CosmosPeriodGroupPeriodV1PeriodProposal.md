# CosmosPeriodGroupPeriodV1PeriodProposal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | id is the unique id of the proposal. | [optional]
**group_policy_address** | Option<**String**> | group_policy_address is the account address of group policy. | [optional]
**metadata** | Option<**String**> | metadata is any arbitrary metadata to attached to the proposal. | [optional]
**proposers** | Option<**Vec<String>**> | proposers are the account addresses of the proposers. | [optional]
**submit_time** | Option<**String**> | submit_time is a timestamp specifying when a proposal was submitted. | [optional]
**group_version** | Option<**String**> | group_version tracks the version of the group at proposal submission. This field is here for informational purposes only. | [optional]
**group_policy_version** | Option<**String**> | group_policy_version tracks the version of the group policy at proposal submission. When a decision policy is changed, existing proposals from previous policy versions will become invalid with the `ABORTED` status. This field is here for informational purposes only. | [optional]
**status** | Option<[**crate::models::CosmosPeriodGroupPeriodV1PeriodProposalStatus**](cosmos.group.v1.ProposalStatus.md)> |  | [optional]
**final_tally_result** | Option<[**crate::models::CosmosPeriodGroupPeriodV1PeriodTallyResult**](cosmos.group.v1.TallyResult.md)> |  | [optional]
**voting_period_end** | Option<**String**> | voting_period_end is the timestamp before which voting must be done. Unless a successfull MsgExec is called before (to execute a proposal whose tally is successful before the voting period ends), tallying will be done at this point, and the `final_tally_result`and `status` fields will be accordingly updated. | [optional]
**executor_result** | Option<[**crate::models::CosmosPeriodGroupPeriodV1PeriodProposalExecutorResult**](cosmos.group.v1.ProposalExecutorResult.md)> |  | [optional]
**messages** | Option<[**Vec<crate::models::GooglePeriodProtobufPeriodAny>**](google.protobuf.Any.md)> | messages is a list of `sdk.Msg`s that will be executed if the proposal passes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


