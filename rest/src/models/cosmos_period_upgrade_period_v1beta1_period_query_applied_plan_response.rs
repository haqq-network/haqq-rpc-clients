/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodUpgradePeriodV1beta1PeriodQueryAppliedPlanResponse : QueryAppliedPlanResponse is the response type for the Query/AppliedPlan RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodUpgradePeriodV1beta1PeriodQueryAppliedPlanResponse {
    /// height is the block height at which the plan was applied.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
}

impl CosmosPeriodUpgradePeriodV1beta1PeriodQueryAppliedPlanResponse {
    /// QueryAppliedPlanResponse is the response type for the Query/AppliedPlan RPC method.
    pub fn new() -> CosmosPeriodUpgradePeriodV1beta1PeriodQueryAppliedPlanResponse {
        CosmosPeriodUpgradePeriodV1beta1PeriodQueryAppliedPlanResponse {
            height: None,
        }
    }
}

