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
pub struct TendermintPeriodAbciPeriodResponseInitChain {
    #[serde(rename = "consensusParams", skip_serializing_if = "Option::is_none")]
    pub consensus_params: Option<Box<crate::models::TendermintPeriodAbciPeriodConsensusParams>>,
    #[serde(rename = "validators", skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<crate::models::TendermintPeriodAbciPeriodValidatorUpdate>>,
    #[serde(rename = "appHash", skip_serializing_if = "Option::is_none")]
    pub app_hash: Option<String>,
}

impl TendermintPeriodAbciPeriodResponseInitChain {
    pub fn new() -> TendermintPeriodAbciPeriodResponseInitChain {
        TendermintPeriodAbciPeriodResponseInitChain {
            consensus_params: None,
            validators: None,
            app_hash: None,
        }
    }
}

