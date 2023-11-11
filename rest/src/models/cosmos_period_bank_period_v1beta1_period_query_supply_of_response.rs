/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBankPeriodV1beta1PeriodQuerySupplyOfResponse : QuerySupplyOfResponse is the response type for the Query/SupplyOf RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBankPeriodV1beta1PeriodQuerySupplyOfResponse {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<crate::models::CosmosPeriodBasePeriodV1beta1PeriodCoin>>,
}

impl CosmosPeriodBankPeriodV1beta1PeriodQuerySupplyOfResponse {
    /// QuerySupplyOfResponse is the response type for the Query/SupplyOf RPC method.
    pub fn new() -> CosmosPeriodBankPeriodV1beta1PeriodQuerySupplyOfResponse {
        CosmosPeriodBankPeriodV1beta1PeriodQuerySupplyOfResponse {
            amount: None,
        }
    }
}


