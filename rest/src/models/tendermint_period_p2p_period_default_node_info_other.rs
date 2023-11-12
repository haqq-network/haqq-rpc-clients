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
pub struct TendermintPeriodP2pPeriodDefaultNodeInfoOther {
    #[serde(rename = "txIndex", skip_serializing_if = "Option::is_none")]
    pub tx_index: Option<String>,
    #[serde(rename = "rpcAddress", skip_serializing_if = "Option::is_none")]
    pub rpc_address: Option<String>,
}

impl TendermintPeriodP2pPeriodDefaultNodeInfoOther {
    pub fn new() -> TendermintPeriodP2pPeriodDefaultNodeInfoOther {
        TendermintPeriodP2pPeriodDefaultNodeInfoOther {
            tx_index: None,
            rpc_address: None,
        }
    }
}

