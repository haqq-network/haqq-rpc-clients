/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodGovPeriodV1PeriodQueryProposalResponse : QueryProposalResponse is the response type for the Query/Proposal RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodGovPeriodV1PeriodQueryProposalResponse {
    #[serde(rename = "proposal", skip_serializing_if = "Option::is_none")]
    pub proposal: Option<Box<crate::models::CosmosPeriodGovPeriodV1PeriodProposal>>,
}

impl CosmosPeriodGovPeriodV1PeriodQueryProposalResponse {
    /// QueryProposalResponse is the response type for the Query/Proposal RPC method.
    pub fn new() -> CosmosPeriodGovPeriodV1PeriodQueryProposalResponse {
        CosmosPeriodGovPeriodV1PeriodQueryProposalResponse {
            proposal: None,
        }
    }
}

