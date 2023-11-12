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
pub struct TendermintPeriodAbciPeriodEvidence {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::TendermintPeriodAbciPeriodEvidenceType>,
    #[serde(rename = "validator", skip_serializing_if = "Option::is_none")]
    pub validator: Option<Box<crate::models::TendermintPeriodAbciPeriodValidator>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "totalVotingPower", skip_serializing_if = "Option::is_none")]
    pub total_voting_power: Option<String>,
}

impl TendermintPeriodAbciPeriodEvidence {
    pub fn new() -> TendermintPeriodAbciPeriodEvidence {
        TendermintPeriodAbciPeriodEvidence {
            r#type: None,
            validator: None,
            height: None,
            time: None,
            total_voting_power: None,
        }
    }
}

