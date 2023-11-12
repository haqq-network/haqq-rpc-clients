/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodGovPeriodV1PeriodVote : Vote defines a vote on a governance proposal. A Vote consists of a proposal ID, the voter, and the vote option.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodGovPeriodV1PeriodVote {
    #[serde(rename = "proposalId", skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    #[serde(rename = "voter", skip_serializing_if = "Option::is_none")]
    pub voter: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<crate::models::CosmosPeriodGovPeriodV1PeriodWeightedVoteOption>>,
    /// metadata is any  arbitrary metadata to attached to the vote.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl CosmosPeriodGovPeriodV1PeriodVote {
    /// Vote defines a vote on a governance proposal. A Vote consists of a proposal ID, the voter, and the vote option.
    pub fn new() -> CosmosPeriodGovPeriodV1PeriodVote {
        CosmosPeriodGovPeriodV1PeriodVote {
            proposal_id: None,
            voter: None,
            options: None,
            metadata: None,
        }
    }
}

