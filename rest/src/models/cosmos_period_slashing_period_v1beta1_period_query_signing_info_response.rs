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
pub struct CosmosPeriodSlashingPeriodV1beta1PeriodQuerySigningInfoResponse {
    #[serde(rename = "valSigningInfo", skip_serializing_if = "Option::is_none")]
    pub val_signing_info: Option<Box<crate::models::CosmosPeriodSlashingPeriodV1beta1PeriodValidatorSigningInfo>>,
}

impl CosmosPeriodSlashingPeriodV1beta1PeriodQuerySigningInfoResponse {
    pub fn new() -> CosmosPeriodSlashingPeriodV1beta1PeriodQuerySigningInfoResponse {
        CosmosPeriodSlashingPeriodV1beta1PeriodQuerySigningInfoResponse {
            val_signing_info: None,
        }
    }
}

