/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodAuthzPeriodV1beta1PeriodQueryGrantsResponse : QueryGrantsResponse is the response type for the Query/Authorizations RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodAuthzPeriodV1beta1PeriodQueryGrantsResponse {
    /// authorizations is a list of grants granted for grantee by granter.
    #[serde(rename = "grants", skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<crate::models::CosmosPeriodAuthzPeriodV1beta1PeriodGrant>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::CosmosPeriodBasePeriodQueryPeriodV1beta1PeriodPageResponse>>,
}

impl CosmosPeriodAuthzPeriodV1beta1PeriodQueryGrantsResponse {
    /// QueryGrantsResponse is the response type for the Query/Authorizations RPC method.
    pub fn new() -> CosmosPeriodAuthzPeriodV1beta1PeriodQueryGrantsResponse {
        CosmosPeriodAuthzPeriodV1beta1PeriodQueryGrantsResponse {
            grants: None,
            pagination: None,
        }
    }
}


