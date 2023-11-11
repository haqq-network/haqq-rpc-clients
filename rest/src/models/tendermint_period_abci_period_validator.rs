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
pub struct TendermintPeriodAbciPeriodValidator {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The voting power
    #[serde(rename = "power", skip_serializing_if = "Option::is_none")]
    pub power: Option<String>,
}

impl TendermintPeriodAbciPeriodValidator {
    pub fn new() -> TendermintPeriodAbciPeriodValidator {
        TendermintPeriodAbciPeriodValidator {
            address: None,
            power: None,
        }
    }
}


