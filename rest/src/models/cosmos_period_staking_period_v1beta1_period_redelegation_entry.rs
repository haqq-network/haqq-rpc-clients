/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodStakingPeriodV1beta1PeriodRedelegationEntry : RedelegationEntry defines a redelegation object with relevant metadata.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodStakingPeriodV1beta1PeriodRedelegationEntry {
    /// creation_height  defines the height which the redelegation took place.
    #[serde(rename = "creationHeight", skip_serializing_if = "Option::is_none")]
    pub creation_height: Option<String>,
    /// completion_time defines the unix time for redelegation completion.
    #[serde(rename = "completionTime", skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<String>,
    /// initial_balance defines the initial balance when redelegation started.
    #[serde(rename = "initialBalance", skip_serializing_if = "Option::is_none")]
    pub initial_balance: Option<String>,
    /// shares_dst is the amount of destination-validator shares created by redelegation.
    #[serde(rename = "sharesDst", skip_serializing_if = "Option::is_none")]
    pub shares_dst: Option<String>,
}

impl CosmosPeriodStakingPeriodV1beta1PeriodRedelegationEntry {
    /// RedelegationEntry defines a redelegation object with relevant metadata.
    pub fn new() -> CosmosPeriodStakingPeriodV1beta1PeriodRedelegationEntry {
        CosmosPeriodStakingPeriodV1beta1PeriodRedelegationEntry {
            creation_height: None,
            completion_time: None,
            initial_balance: None,
            shares_dst: None,
        }
    }
}

