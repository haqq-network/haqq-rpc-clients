/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodTxPeriodV1beta1PeriodTx : Tx is the standard type used for broadcasting transactions.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodTxPeriodV1beta1PeriodTx {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<crate::models::CosmosPeriodTxPeriodV1beta1PeriodTxBody>>,
    #[serde(rename = "authInfo", skip_serializing_if = "Option::is_none")]
    pub auth_info: Option<Box<crate::models::CosmosPeriodTxPeriodV1beta1PeriodAuthInfo>>,
    /// signatures is a list of signatures that matches the length and order of AuthInfo's signer_infos to allow connecting signature meta information like public key and signing mode by position.
    #[serde(rename = "signatures", skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<String>>,
}

impl CosmosPeriodTxPeriodV1beta1PeriodTx {
    /// Tx is the standard type used for broadcasting transactions.
    pub fn new() -> CosmosPeriodTxPeriodV1beta1PeriodTx {
        CosmosPeriodTxPeriodV1beta1PeriodTx {
            body: None,
            auth_info: None,
            signatures: None,
        }
    }
}

