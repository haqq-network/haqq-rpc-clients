/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBankPeriodV1beta1PeriodQueryAllBalancesResponse : QueryAllBalancesResponse is the response type for the Query/AllBalances RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBankPeriodV1beta1PeriodQueryAllBalancesResponse {
    /// balances is the balances of all the coins.
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none")]
    pub balances: Option<Vec<crate::models::CosmosPeriodBasePeriodV1beta1PeriodCoin>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::CosmosPeriodBasePeriodQueryPeriodV1beta1PeriodPageResponse>>,
}

impl CosmosPeriodBankPeriodV1beta1PeriodQueryAllBalancesResponse {
    /// QueryAllBalancesResponse is the response type for the Query/AllBalances RPC method.
    pub fn new() -> CosmosPeriodBankPeriodV1beta1PeriodQueryAllBalancesResponse {
        CosmosPeriodBankPeriodV1beta1PeriodQueryAllBalancesResponse {
            balances: None,
            pagination: None,
        }
    }
}

