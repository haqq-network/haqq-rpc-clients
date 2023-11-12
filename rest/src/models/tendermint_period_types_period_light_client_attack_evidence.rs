/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TendermintPeriodTypesPeriodLightClientAttackEvidence : LightClientAttackEvidence contains evidence of a set of validators attempting to mislead a light client.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TendermintPeriodTypesPeriodLightClientAttackEvidence {
    #[serde(rename = "conflictingBlock", skip_serializing_if = "Option::is_none")]
    pub conflicting_block: Option<Box<crate::models::TendermintPeriodTypesPeriodLightBlock>>,
    #[serde(rename = "commonHeight", skip_serializing_if = "Option::is_none")]
    pub common_height: Option<String>,
    #[serde(rename = "byzantineValidators", skip_serializing_if = "Option::is_none")]
    pub byzantine_validators: Option<Vec<crate::models::TendermintPeriodTypesPeriodValidator>>,
    #[serde(rename = "totalVotingPower", skip_serializing_if = "Option::is_none")]
    pub total_voting_power: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl TendermintPeriodTypesPeriodLightClientAttackEvidence {
    /// LightClientAttackEvidence contains evidence of a set of validators attempting to mislead a light client.
    pub fn new() -> TendermintPeriodTypesPeriodLightClientAttackEvidence {
        TendermintPeriodTypesPeriodLightClientAttackEvidence {
            conflicting_block: None,
            common_height: None,
            byzantine_validators: None,
            total_voting_power: None,
            timestamp: None,
        }
    }
}

