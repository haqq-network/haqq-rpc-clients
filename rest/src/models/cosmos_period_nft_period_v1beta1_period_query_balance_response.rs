/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodNftPeriodV1beta1PeriodQueryBalanceResponse {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl CosmosPeriodNftPeriodV1beta1PeriodQueryBalanceResponse {
    pub fn new() -> CosmosPeriodNftPeriodV1beta1PeriodQueryBalanceResponse {
        CosmosPeriodNftPeriodV1beta1PeriodQueryBalanceResponse {
            amount: None,
        }
    }
}


