/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TendermintPeriodTypesPeriodVote : Vote represents a prevote, precommit, or commit vote from validators for consensus.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TendermintPeriodTypesPeriodVote {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::TendermintPeriodTypesPeriodSignedMsgType>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "round", skip_serializing_if = "Option::is_none")]
    pub round: Option<i32>,
    #[serde(rename = "blockId", skip_serializing_if = "Option::is_none")]
    pub block_id: Option<Box<crate::models::TendermintPeriodTypesPeriodBlockId>>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "validatorAddress", skip_serializing_if = "Option::is_none")]
    pub validator_address: Option<String>,
    #[serde(rename = "validatorIndex", skip_serializing_if = "Option::is_none")]
    pub validator_index: Option<i32>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl TendermintPeriodTypesPeriodVote {
    /// Vote represents a prevote, precommit, or commit vote from validators for consensus.
    pub fn new() -> TendermintPeriodTypesPeriodVote {
        TendermintPeriodTypesPeriodVote {
            r#type: None,
            height: None,
            round: None,
            block_id: None,
            timestamp: None,
            validator_address: None,
            validator_index: None,
            signature: None,
        }
    }
}

