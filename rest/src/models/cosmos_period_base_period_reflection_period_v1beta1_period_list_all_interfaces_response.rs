/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBasePeriodReflectionPeriodV1beta1PeriodListAllInterfacesResponse : ListAllInterfacesResponse is the response type of the ListAllInterfaces RPC.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBasePeriodReflectionPeriodV1beta1PeriodListAllInterfacesResponse {
    /// interface_names is an array of all the registered interfaces.
    #[serde(rename = "interfaceNames", skip_serializing_if = "Option::is_none")]
    pub interface_names: Option<Vec<String>>,
}

impl CosmosPeriodBasePeriodReflectionPeriodV1beta1PeriodListAllInterfacesResponse {
    /// ListAllInterfacesResponse is the response type of the ListAllInterfaces RPC.
    pub fn new() -> CosmosPeriodBasePeriodReflectionPeriodV1beta1PeriodListAllInterfacesResponse {
        CosmosPeriodBasePeriodReflectionPeriodV1beta1PeriodListAllInterfacesResponse {
            interface_names: None,
        }
    }
}

