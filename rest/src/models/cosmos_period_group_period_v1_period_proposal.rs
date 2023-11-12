/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodGroupPeriodV1PeriodProposal : Proposal defines a group proposal. Any member of a group can submit a proposal for a group policy to decide upon. A proposal consists of a set of `sdk.Msg`s that will be executed if the proposal passes as well as some optional metadata associated with the proposal.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodGroupPeriodV1PeriodProposal {
    /// id is the unique id of the proposal.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// group_policy_address is the account address of group policy.
    #[serde(rename = "groupPolicyAddress", skip_serializing_if = "Option::is_none")]
    pub group_policy_address: Option<String>,
    /// metadata is any arbitrary metadata to attached to the proposal.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// proposers are the account addresses of the proposers.
    #[serde(rename = "proposers", skip_serializing_if = "Option::is_none")]
    pub proposers: Option<Vec<String>>,
    /// submit_time is a timestamp specifying when a proposal was submitted.
    #[serde(rename = "submitTime", skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    /// group_version tracks the version of the group at proposal submission. This field is here for informational purposes only.
    #[serde(rename = "groupVersion", skip_serializing_if = "Option::is_none")]
    pub group_version: Option<String>,
    /// group_policy_version tracks the version of the group policy at proposal submission. When a decision policy is changed, existing proposals from previous policy versions will become invalid with the `ABORTED` status. This field is here for informational purposes only.
    #[serde(rename = "groupPolicyVersion", skip_serializing_if = "Option::is_none")]
    pub group_policy_version: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::CosmosPeriodGroupPeriodV1PeriodProposalStatus>,
    #[serde(rename = "finalTallyResult", skip_serializing_if = "Option::is_none")]
    pub final_tally_result: Option<Box<crate::models::CosmosPeriodGroupPeriodV1PeriodTallyResult>>,
    /// voting_period_end is the timestamp before which voting must be done. Unless a successfull MsgExec is called before (to execute a proposal whose tally is successful before the voting period ends), tallying will be done at this point, and the `final_tally_result`and `status` fields will be accordingly updated.
    #[serde(rename = "votingPeriodEnd", skip_serializing_if = "Option::is_none")]
    pub voting_period_end: Option<String>,
    #[serde(rename = "executorResult", skip_serializing_if = "Option::is_none")]
    pub executor_result: Option<crate::models::CosmosPeriodGroupPeriodV1PeriodProposalExecutorResult>,
    /// messages is a list of `sdk.Msg`s that will be executed if the proposal passes.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::models::GooglePeriodProtobufPeriodAny>>,
}

impl CosmosPeriodGroupPeriodV1PeriodProposal {
    /// Proposal defines a group proposal. Any member of a group can submit a proposal for a group policy to decide upon. A proposal consists of a set of `sdk.Msg`s that will be executed if the proposal passes as well as some optional metadata associated with the proposal.
    pub fn new() -> CosmosPeriodGroupPeriodV1PeriodProposal {
        CosmosPeriodGroupPeriodV1PeriodProposal {
            id: None,
            group_policy_address: None,
            metadata: None,
            proposers: None,
            submit_time: None,
            group_version: None,
            group_policy_version: None,
            status: None,
            final_tally_result: None,
            voting_period_end: None,
            executor_result: None,
            messages: None,
        }
    }
}

