/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TendermintPeriodTypesPeriodBlockIdFlag {
    #[serde(rename = "BLOCK_ID_FLAG_UNKNOWN")]
    Unknown,
    #[serde(rename = "BLOCK_ID_FLAG_ABSENT")]
    Absent,
    #[serde(rename = "BLOCK_ID_FLAG_COMMIT")]
    Commit,
    #[serde(rename = "BLOCK_ID_FLAG_NIL")]
    Nil,

}

impl ToString for TendermintPeriodTypesPeriodBlockIdFlag {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("BLOCK_ID_FLAG_UNKNOWN"),
            Self::Absent => String::from("BLOCK_ID_FLAG_ABSENT"),
            Self::Commit => String::from("BLOCK_ID_FLAG_COMMIT"),
            Self::Nil => String::from("BLOCK_ID_FLAG_NIL"),
        }
    }
}

impl Default for TendermintPeriodTypesPeriodBlockIdFlag {
    fn default() -> TendermintPeriodTypesPeriodBlockIdFlag {
        Self::Unknown
    }
}




