/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBasePeriodQueryPeriodV1beta1PeriodPageResponse : PageResponse is to be embedded in gRPC response messages where the corresponding request message has used PageRequest.   message SomeResponse {          repeated Bar results = 1;          PageResponse page = 2;  }



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBasePeriodQueryPeriodV1beta1PeriodPageResponse {
    /// next_key is the key to be passed to PageRequest.key to query the next page most efficiently. It will be empty if there are no more results.
    #[serde(rename = "nextKey", skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
}

impl CosmosPeriodBasePeriodQueryPeriodV1beta1PeriodPageResponse {
    /// PageResponse is to be embedded in gRPC response messages where the corresponding request message has used PageRequest.   message SomeResponse {          repeated Bar results = 1;          PageResponse page = 2;  }
    pub fn new() -> CosmosPeriodBasePeriodQueryPeriodV1beta1PeriodPageResponse {
        CosmosPeriodBasePeriodQueryPeriodV1beta1PeriodPageResponse {
            next_key: None,
            total: None,
        }
    }
}


