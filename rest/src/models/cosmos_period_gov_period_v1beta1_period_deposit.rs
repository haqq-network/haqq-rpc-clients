/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodGovPeriodV1beta1PeriodDeposit : Deposit defines an amount deposited by an account address to an active proposal.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodGovPeriodV1beta1PeriodDeposit {
    #[serde(rename = "proposalId", skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    #[serde(rename = "depositor", skip_serializing_if = "Option::is_none")]
    pub depositor: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Vec<crate::models::CosmosPeriodBasePeriodV1beta1PeriodCoin>>,
}

impl CosmosPeriodGovPeriodV1beta1PeriodDeposit {
    /// Deposit defines an amount deposited by an account address to an active proposal.
    pub fn new() -> CosmosPeriodGovPeriodV1beta1PeriodDeposit {
        CosmosPeriodGovPeriodV1beta1PeriodDeposit {
            proposal_id: None,
            depositor: None,
            amount: None,
        }
    }
}


