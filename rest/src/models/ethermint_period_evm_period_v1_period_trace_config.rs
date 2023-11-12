/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EthermintPeriodEvmPeriodV1PeriodTraceConfig : TraceConfig holds extra parameters to trace functions.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EthermintPeriodEvmPeriodV1PeriodTraceConfig {
    #[serde(rename = "tracer", skip_serializing_if = "Option::is_none")]
    pub tracer: Option<String>,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(rename = "reexec", skip_serializing_if = "Option::is_none")]
    pub reexec: Option<String>,
    #[serde(rename = "disableStack", skip_serializing_if = "Option::is_none")]
    pub disable_stack: Option<bool>,
    #[serde(rename = "disableStorage", skip_serializing_if = "Option::is_none")]
    pub disable_storage: Option<bool>,
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "overrides", skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Box<crate::models::EthermintPeriodEvmPeriodV1PeriodChainConfig>>,
    #[serde(rename = "enableMemory", skip_serializing_if = "Option::is_none")]
    pub enable_memory: Option<bool>,
    #[serde(rename = "enableReturnData", skip_serializing_if = "Option::is_none")]
    pub enable_return_data: Option<bool>,
    #[serde(rename = "tracerJsonConfig", skip_serializing_if = "Option::is_none")]
    pub tracer_json_config: Option<String>,
}

impl EthermintPeriodEvmPeriodV1PeriodTraceConfig {
    /// TraceConfig holds extra parameters to trace functions.
    pub fn new() -> EthermintPeriodEvmPeriodV1PeriodTraceConfig {
        EthermintPeriodEvmPeriodV1PeriodTraceConfig {
            tracer: None,
            timeout: None,
            reexec: None,
            disable_stack: None,
            disable_storage: None,
            debug: None,
            limit: None,
            overrides: None,
            enable_memory: None,
            enable_return_data: None,
            tracer_json_config: None,
        }
    }
}

