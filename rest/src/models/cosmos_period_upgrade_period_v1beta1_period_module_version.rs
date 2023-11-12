/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodUpgradePeriodV1beta1PeriodModuleVersion : ModuleVersion specifies a module and its consensus version.  Since: cosmos-sdk 0.43



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodUpgradePeriodV1beta1PeriodModuleVersion {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl CosmosPeriodUpgradePeriodV1beta1PeriodModuleVersion {
    /// ModuleVersion specifies a module and its consensus version.  Since: cosmos-sdk 0.43
    pub fn new() -> CosmosPeriodUpgradePeriodV1beta1PeriodModuleVersion {
        CosmosPeriodUpgradePeriodV1beta1PeriodModuleVersion {
            name: None,
            version: None,
        }
    }
}

