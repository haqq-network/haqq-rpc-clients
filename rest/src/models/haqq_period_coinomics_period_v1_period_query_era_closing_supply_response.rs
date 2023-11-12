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
pub struct HaqqPeriodCoinomicsPeriodV1PeriodQueryEraClosingSupplyResponse {
    #[serde(rename = "eraClosingSupply", skip_serializing_if = "Option::is_none")]
    pub era_closing_supply: Option<Box<crate::models::CosmosPeriodBasePeriodV1beta1PeriodCoin>>,
}

impl HaqqPeriodCoinomicsPeriodV1PeriodQueryEraClosingSupplyResponse {
    pub fn new() -> HaqqPeriodCoinomicsPeriodV1PeriodQueryEraClosingSupplyResponse {
        HaqqPeriodCoinomicsPeriodV1PeriodQueryEraClosingSupplyResponse {
            era_closing_supply: None,
        }
    }
}

