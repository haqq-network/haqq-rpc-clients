/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorOutstandingRewardsResponse : QueryValidatorOutstandingRewardsResponse is the response type for the Query/ValidatorOutstandingRewards RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorOutstandingRewardsResponse {
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Box<crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodValidatorOutstandingRewards>>,
}

impl CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorOutstandingRewardsResponse {
    /// QueryValidatorOutstandingRewardsResponse is the response type for the Query/ValidatorOutstandingRewards RPC method.
    pub fn new() -> CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorOutstandingRewardsResponse {
        CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorOutstandingRewardsResponse {
            rewards: None,
        }
    }
}


