/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodUpgradePeriodV1beta1PeriodQueryCurrentPlanResponse : QueryCurrentPlanResponse is the response type for the Query/CurrentPlan RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodUpgradePeriodV1beta1PeriodQueryCurrentPlanResponse {
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodPlan>>,
}

impl CosmosPeriodUpgradePeriodV1beta1PeriodQueryCurrentPlanResponse {
    /// QueryCurrentPlanResponse is the response type for the Query/CurrentPlan RPC method.
    pub fn new() -> CosmosPeriodUpgradePeriodV1beta1PeriodQueryCurrentPlanResponse {
        CosmosPeriodUpgradePeriodV1beta1PeriodQueryCurrentPlanResponse {
            plan: None,
        }
    }
}


