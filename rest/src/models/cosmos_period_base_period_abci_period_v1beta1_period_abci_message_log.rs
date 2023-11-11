/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBasePeriodAbciPeriodV1beta1PeriodAbciMessageLog : ABCIMessageLog defines a structure containing an indexed tx ABCI message log.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBasePeriodAbciPeriodV1beta1PeriodAbciMessageLog {
    #[serde(rename = "msgIndex", skip_serializing_if = "Option::is_none")]
    pub msg_index: Option<i64>,
    #[serde(rename = "log", skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    /// Events contains a slice of Event objects that were emitted during some execution.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::CosmosPeriodBasePeriodAbciPeriodV1beta1PeriodStringEvent>>,
}

impl CosmosPeriodBasePeriodAbciPeriodV1beta1PeriodAbciMessageLog {
    /// ABCIMessageLog defines a structure containing an indexed tx ABCI message log.
    pub fn new() -> CosmosPeriodBasePeriodAbciPeriodV1beta1PeriodAbciMessageLog {
        CosmosPeriodBasePeriodAbciPeriodV1beta1PeriodAbciMessageLog {
            msg_index: None,
            log: None,
            events: None,
        }
    }
}


