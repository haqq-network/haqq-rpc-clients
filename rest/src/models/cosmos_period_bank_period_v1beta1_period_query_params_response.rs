/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBankPeriodV1beta1PeriodQueryParamsResponse : QueryParamsResponse defines the response type for querying x/bank parameters.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBankPeriodV1beta1PeriodQueryParamsResponse {
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Box<crate::models::CosmosPeriodBankPeriodV1beta1PeriodParams>>,
}

impl CosmosPeriodBankPeriodV1beta1PeriodQueryParamsResponse {
    /// QueryParamsResponse defines the response type for querying x/bank parameters.
    pub fn new() -> CosmosPeriodBankPeriodV1beta1PeriodQueryParamsResponse {
        CosmosPeriodBankPeriodV1beta1PeriodQueryParamsResponse {
            params: None,
        }
    }
}

