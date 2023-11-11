/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBankPeriodV1beta1PeriodQueryDenomMetadataResponse : QueryDenomMetadataResponse is the response type for the Query/DenomMetadata RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBankPeriodV1beta1PeriodQueryDenomMetadataResponse {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::CosmosPeriodBankPeriodV1beta1PeriodMetadata>>,
}

impl CosmosPeriodBankPeriodV1beta1PeriodQueryDenomMetadataResponse {
    /// QueryDenomMetadataResponse is the response type for the Query/DenomMetadata RPC method.
    pub fn new() -> CosmosPeriodBankPeriodV1beta1PeriodQueryDenomMetadataResponse {
        CosmosPeriodBankPeriodV1beta1PeriodQueryDenomMetadataResponse {
            metadata: None,
        }
    }
}


