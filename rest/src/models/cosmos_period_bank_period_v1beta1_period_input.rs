/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBankPeriodV1beta1PeriodInput : Input models transaction input.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBankPeriodV1beta1PeriodInput {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "coins", skip_serializing_if = "Option::is_none")]
    pub coins: Option<Vec<crate::models::CosmosPeriodBasePeriodV1beta1PeriodCoin>>,
}

impl CosmosPeriodBankPeriodV1beta1PeriodInput {
    /// Input models transaction input.
    pub fn new() -> CosmosPeriodBankPeriodV1beta1PeriodInput {
        CosmosPeriodBankPeriodV1beta1PeriodInput {
            address: None,
            coins: None,
        }
    }
}

