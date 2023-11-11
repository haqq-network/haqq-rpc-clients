/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodStakingPeriodV1beta1PeriodDelegation : Delegation represents the bond with tokens held by an account. It is owned by one delegator, and is associated with the voting power of one validator.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodStakingPeriodV1beta1PeriodDelegation {
    /// delegator_address is the bech32-encoded address of the delegator.
    #[serde(rename = "delegatorAddress", skip_serializing_if = "Option::is_none")]
    pub delegator_address: Option<String>,
    /// validator_address is the bech32-encoded address of the validator.
    #[serde(rename = "validatorAddress", skip_serializing_if = "Option::is_none")]
    pub validator_address: Option<String>,
    /// shares define the delegation shares received.
    #[serde(rename = "shares", skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
}

impl CosmosPeriodStakingPeriodV1beta1PeriodDelegation {
    /// Delegation represents the bond with tokens held by an account. It is owned by one delegator, and is associated with the voting power of one validator.
    pub fn new() -> CosmosPeriodStakingPeriodV1beta1PeriodDelegation {
        CosmosPeriodStakingPeriodV1beta1PeriodDelegation {
            delegator_address: None,
            validator_address: None,
            shares: None,
        }
    }
}


