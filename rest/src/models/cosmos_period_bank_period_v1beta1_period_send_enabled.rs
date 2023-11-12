/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBankPeriodV1beta1PeriodSendEnabled : SendEnabled maps coin denom to a send_enabled status (whether a denom is sendable).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBankPeriodV1beta1PeriodSendEnabled {
    #[serde(rename = "denom", skip_serializing_if = "Option::is_none")]
    pub denom: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl CosmosPeriodBankPeriodV1beta1PeriodSendEnabled {
    /// SendEnabled maps coin denom to a send_enabled status (whether a denom is sendable).
    pub fn new() -> CosmosPeriodBankPeriodV1beta1PeriodSendEnabled {
        CosmosPeriodBankPeriodV1beta1PeriodSendEnabled {
            denom: None,
            enabled: None,
        }
    }
}

