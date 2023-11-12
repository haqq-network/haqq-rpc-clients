/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodGovPeriodV1beta1PeriodProposal : Proposal defines the core field members of a governance proposal.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodGovPeriodV1beta1PeriodProposal {
    #[serde(rename = "proposalId", skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::models::GooglePeriodProtobufPeriodAny>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::CosmosPeriodGovPeriodV1beta1PeriodProposalStatus>,
    #[serde(rename = "finalTallyResult", skip_serializing_if = "Option::is_none")]
    pub final_tally_result: Option<Box<crate::models::CosmosPeriodGovPeriodV1beta1PeriodTallyResult>>,
    #[serde(rename = "submitTime", skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    #[serde(rename = "depositEndTime", skip_serializing_if = "Option::is_none")]
    pub deposit_end_time: Option<String>,
    #[serde(rename = "totalDeposit", skip_serializing_if = "Option::is_none")]
    pub total_deposit: Option<Vec<crate::models::CosmosPeriodBasePeriodV1beta1PeriodCoin>>,
    #[serde(rename = "votingStartTime", skip_serializing_if = "Option::is_none")]
    pub voting_start_time: Option<String>,
    #[serde(rename = "votingEndTime", skip_serializing_if = "Option::is_none")]
    pub voting_end_time: Option<String>,
}

impl CosmosPeriodGovPeriodV1beta1PeriodProposal {
    /// Proposal defines the core field members of a governance proposal.
    pub fn new() -> CosmosPeriodGovPeriodV1beta1PeriodProposal {
        CosmosPeriodGovPeriodV1beta1PeriodProposal {
            proposal_id: None,
            content: None,
            status: None,
            final_tally_result: None,
            submit_time: None,
            deposit_end_time: None,
            total_deposit: None,
            voting_start_time: None,
            voting_end_time: None,
        }
    }
}

