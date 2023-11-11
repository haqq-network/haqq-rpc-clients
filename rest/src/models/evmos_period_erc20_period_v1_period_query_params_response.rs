/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EvmosPeriodErc20PeriodV1PeriodQueryParamsResponse : QueryParamsResponse is the response type for the Query/Params RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvmosPeriodErc20PeriodV1PeriodQueryParamsResponse {
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Box<crate::models::EvmosPeriodErc20PeriodV1PeriodParams>>,
}

impl EvmosPeriodErc20PeriodV1PeriodQueryParamsResponse {
    /// QueryParamsResponse is the response type for the Query/Params RPC method.
    pub fn new() -> EvmosPeriodErc20PeriodV1PeriodQueryParamsResponse {
        EvmosPeriodErc20PeriodV1PeriodQueryParamsResponse {
            params: None,
        }
    }
}


