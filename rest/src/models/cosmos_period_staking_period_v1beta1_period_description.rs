/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodStakingPeriodV1beta1PeriodDescription : Description defines a validator description.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodStakingPeriodV1beta1PeriodDescription {
    /// moniker defines a human-readable name for the validator.
    #[serde(rename = "moniker", skip_serializing_if = "Option::is_none")]
    pub moniker: Option<String>,
    /// identity defines an optional identity signature (ex. UPort or Keybase).
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// website defines an optional website link.
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// security_contact defines an optional email for security contact.
    #[serde(rename = "securityContact", skip_serializing_if = "Option::is_none")]
    pub security_contact: Option<String>,
    /// details define other optional details.
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl CosmosPeriodStakingPeriodV1beta1PeriodDescription {
    /// Description defines a validator description.
    pub fn new() -> CosmosPeriodStakingPeriodV1beta1PeriodDescription {
        CosmosPeriodStakingPeriodV1beta1PeriodDescription {
            moniker: None,
            identity: None,
            website: None,
            security_contact: None,
            details: None,
        }
    }
}


