/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodGroupPeriodV1PeriodQueryProposalResponse : QueryProposalResponse is the Query/Proposal response type.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodGroupPeriodV1PeriodQueryProposalResponse {
    #[serde(rename = "proposal", skip_serializing_if = "Option::is_none")]
    pub proposal: Option<Box<crate::models::CosmosPeriodGroupPeriodV1PeriodProposal>>,
}

impl CosmosPeriodGroupPeriodV1PeriodQueryProposalResponse {
    /// QueryProposalResponse is the Query/Proposal response type.
    pub fn new() -> CosmosPeriodGroupPeriodV1PeriodQueryProposalResponse {
        CosmosPeriodGroupPeriodV1PeriodQueryProposalResponse {
            proposal: None,
        }
    }
}


