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
pub struct TendermintPeriodAbciPeriodResponseListSnapshots {
    #[serde(rename = "snapshots", skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<crate::models::TendermintPeriodAbciPeriodSnapshot>>,
}

impl TendermintPeriodAbciPeriodResponseListSnapshots {
    pub fn new() -> TendermintPeriodAbciPeriodResponseListSnapshots {
        TendermintPeriodAbciPeriodResponseListSnapshots {
            snapshots: None,
        }
    }
}


