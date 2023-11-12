/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodDistributionPeriodV1beta1PeriodParams : Params defines the set of params for the distribution module.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodDistributionPeriodV1beta1PeriodParams {
    #[serde(rename = "communityTax", skip_serializing_if = "Option::is_none")]
    pub community_tax: Option<String>,
    #[serde(rename = "baseProposerReward", skip_serializing_if = "Option::is_none")]
    pub base_proposer_reward: Option<String>,
    #[serde(rename = "bonusProposerReward", skip_serializing_if = "Option::is_none")]
    pub bonus_proposer_reward: Option<String>,
    #[serde(rename = "withdrawAddrEnabled", skip_serializing_if = "Option::is_none")]
    pub withdraw_addr_enabled: Option<bool>,
}

impl CosmosPeriodDistributionPeriodV1beta1PeriodParams {
    /// Params defines the set of params for the distribution module.
    pub fn new() -> CosmosPeriodDistributionPeriodV1beta1PeriodParams {
        CosmosPeriodDistributionPeriodV1beta1PeriodParams {
            community_tax: None,
            base_proposer_reward: None,
            bonus_proposer_reward: None,
            withdraw_addr_enabled: None,
        }
    }
}

