/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetValidatorSetByHeightResponse : GetValidatorSetByHeightResponse is the response type for the Query/GetValidatorSetByHeight RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetValidatorSetByHeightResponse {
    #[serde(rename = "blockHeight", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<String>,
    #[serde(rename = "validators", skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodValidator>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::CosmosPeriodBasePeriodQueryPeriodV1beta1PeriodPageResponse>>,
}

impl CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetValidatorSetByHeightResponse {
    /// GetValidatorSetByHeightResponse is the response type for the Query/GetValidatorSetByHeight RPC method.
    pub fn new() -> CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetValidatorSetByHeightResponse {
        CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetValidatorSetByHeightResponse {
            block_height: None,
            validators: None,
            pagination: None,
        }
    }
}


