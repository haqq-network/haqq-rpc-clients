/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBasePeriodV1beta1PeriodCoin : Coin defines a token with a denomination and an amount.  NOTE: The amount field is an Int which implements the custom method signatures required by gogoproto.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBasePeriodV1beta1PeriodCoin {
    #[serde(rename = "denom", skip_serializing_if = "Option::is_none")]
    pub denom: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl CosmosPeriodBasePeriodV1beta1PeriodCoin {
    /// Coin defines a token with a denomination and an amount.  NOTE: The amount field is an Int which implements the custom method signatures required by gogoproto.
    pub fn new() -> CosmosPeriodBasePeriodV1beta1PeriodCoin {
        CosmosPeriodBasePeriodV1beta1PeriodCoin {
            denom: None,
            amount: None,
        }
    }
}


