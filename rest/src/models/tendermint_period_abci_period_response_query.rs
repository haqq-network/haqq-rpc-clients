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
pub struct TendermintPeriodAbciPeriodResponseQuery {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// bytes data = 2; // use \"value\" instead.  nondeterministic
    #[serde(rename = "log", skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "proofOps", skip_serializing_if = "Option::is_none")]
    pub proof_ops: Option<Box<crate::models::TendermintPeriodCryptoPeriodProofOps>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "codespace", skip_serializing_if = "Option::is_none")]
    pub codespace: Option<String>,
}

impl TendermintPeriodAbciPeriodResponseQuery {
    pub fn new() -> TendermintPeriodAbciPeriodResponseQuery {
        TendermintPeriodAbciPeriodResponseQuery {
            code: None,
            log: None,
            info: None,
            index: None,
            key: None,
            value: None,
            proof_ops: None,
            height: None,
            codespace: None,
        }
    }
}


