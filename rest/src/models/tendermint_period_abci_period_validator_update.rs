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
pub struct TendermintPeriodAbciPeriodValidatorUpdate {
    #[serde(rename = "pubKey", skip_serializing_if = "Option::is_none")]
    pub pub_key: Option<Box<crate::models::TendermintPeriodCryptoPeriodPublicKey>>,
    #[serde(rename = "power", skip_serializing_if = "Option::is_none")]
    pub power: Option<String>,
}

impl TendermintPeriodAbciPeriodValidatorUpdate {
    pub fn new() -> TendermintPeriodAbciPeriodValidatorUpdate {
        TendermintPeriodAbciPeriodValidatorUpdate {
            pub_key: None,
            power: None,
        }
    }
}


