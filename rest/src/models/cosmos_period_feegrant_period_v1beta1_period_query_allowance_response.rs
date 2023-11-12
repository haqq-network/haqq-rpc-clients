/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowanceResponse : QueryAllowanceResponse is the response type for the Query/Allowance RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowanceResponse {
    #[serde(rename = "allowance", skip_serializing_if = "Option::is_none")]
    pub allowance: Option<Box<crate::models::CosmosPeriodFeegrantPeriodV1beta1PeriodGrant>>,
}

impl CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowanceResponse {
    /// QueryAllowanceResponse is the response type for the Query/Allowance RPC method.
    pub fn new() -> CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowanceResponse {
        CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowanceResponse {
            allowance: None,
        }
    }
}

