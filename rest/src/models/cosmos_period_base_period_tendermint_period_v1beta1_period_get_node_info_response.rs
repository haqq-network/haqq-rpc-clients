/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetNodeInfoResponse : GetNodeInfoResponse is the response type for the Query/GetNodeInfo RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetNodeInfoResponse {
    #[serde(rename = "defaultNodeInfo", skip_serializing_if = "Option::is_none")]
    pub default_node_info: Option<Box<crate::models::TendermintPeriodP2pPeriodDefaultNodeInfo>>,
    #[serde(rename = "applicationVersion", skip_serializing_if = "Option::is_none")]
    pub application_version: Option<Box<crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodVersionInfo>>,
}

impl CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetNodeInfoResponse {
    /// GetNodeInfoResponse is the response type for the Query/GetNodeInfo RPC method.
    pub fn new() -> CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetNodeInfoResponse {
        CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetNodeInfoResponse {
            default_node_info: None,
            application_version: None,
        }
    }
}

