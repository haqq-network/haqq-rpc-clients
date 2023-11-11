/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountResponse : QueryAccountResponse is the response type for the Query/Account RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountResponse {
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::models::GooglePeriodProtobufPeriodAny>,
}

impl CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountResponse {
    /// QueryAccountResponse is the response type for the Query/Account RPC method.
    pub fn new() -> CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountResponse {
        CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountResponse {
            account: None,
        }
    }
}


