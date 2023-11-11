/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EthermintPeriodEvmPeriodV1PeriodQueryCodeResponse : QueryCodeResponse is the response type for the Query/Code RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EthermintPeriodEvmPeriodV1PeriodQueryCodeResponse {
    /// code represents the code bytes from an ethereum address.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl EthermintPeriodEvmPeriodV1PeriodQueryCodeResponse {
    /// QueryCodeResponse is the response type for the Query/Code RPC method.
    pub fn new() -> EthermintPeriodEvmPeriodV1PeriodQueryCodeResponse {
        EthermintPeriodEvmPeriodV1PeriodQueryCodeResponse {
            code: None,
        }
    }
}


