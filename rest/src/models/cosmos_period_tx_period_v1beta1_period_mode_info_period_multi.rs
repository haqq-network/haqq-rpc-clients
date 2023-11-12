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
pub struct CosmosPeriodTxPeriodV1beta1PeriodModeInfoPeriodMulti {
    #[serde(rename = "bitarray", skip_serializing_if = "Option::is_none")]
    pub bitarray: Option<Box<crate::models::CosmosPeriodCryptoPeriodMultisigPeriodV1beta1PeriodCompactBitArray>>,
    #[serde(rename = "modeInfos", skip_serializing_if = "Option::is_none")]
    pub mode_infos: Option<Vec<crate::models::CosmosPeriodTxPeriodV1beta1PeriodModeInfo>>,
}

impl CosmosPeriodTxPeriodV1beta1PeriodModeInfoPeriodMulti {
    pub fn new() -> CosmosPeriodTxPeriodV1beta1PeriodModeInfoPeriodMulti {
        CosmosPeriodTxPeriodV1beta1PeriodModeInfoPeriodMulti {
            bitarray: None,
            mode_infos: None,
        }
    }
}

