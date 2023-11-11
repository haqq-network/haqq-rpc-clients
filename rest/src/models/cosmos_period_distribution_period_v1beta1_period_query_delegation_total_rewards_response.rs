/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegationTotalRewardsResponse : QueryDelegationTotalRewardsResponse is the response type for the Query/DelegationTotalRewards RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegationTotalRewardsResponse {
    /// rewards defines all the rewards accrued by a delegator.
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodDelegationDelegatorReward>>,
    /// total defines the sum of all the rewards.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<Vec<crate::models::CosmosPeriodBasePeriodV1beta1PeriodDecCoin>>,
}

impl CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegationTotalRewardsResponse {
    /// QueryDelegationTotalRewardsResponse is the response type for the Query/DelegationTotalRewards RPC method.
    pub fn new() -> CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegationTotalRewardsResponse {
        CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegationTotalRewardsResponse {
            rewards: None,
            total: None,
        }
    }
}


