/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodMintPeriodV1beta1PeriodParams : Params holds parameters for the mint module.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodMintPeriodV1beta1PeriodParams {
    #[serde(rename = "mintDenom", skip_serializing_if = "Option::is_none")]
    pub mint_denom: Option<String>,
    #[serde(rename = "inflationRateChange", skip_serializing_if = "Option::is_none")]
    pub inflation_rate_change: Option<String>,
    #[serde(rename = "inflationMax", skip_serializing_if = "Option::is_none")]
    pub inflation_max: Option<String>,
    #[serde(rename = "inflationMin", skip_serializing_if = "Option::is_none")]
    pub inflation_min: Option<String>,
    #[serde(rename = "goalBonded", skip_serializing_if = "Option::is_none")]
    pub goal_bonded: Option<String>,
    #[serde(rename = "blocksPerYear", skip_serializing_if = "Option::is_none")]
    pub blocks_per_year: Option<String>,
}

impl CosmosPeriodMintPeriodV1beta1PeriodParams {
    /// Params holds parameters for the mint module.
    pub fn new() -> CosmosPeriodMintPeriodV1beta1PeriodParams {
        CosmosPeriodMintPeriodV1beta1PeriodParams {
            mint_denom: None,
            inflation_rate_change: None,
            inflation_max: None,
            inflation_min: None,
            goal_bonded: None,
            blocks_per_year: None,
        }
    }
}


