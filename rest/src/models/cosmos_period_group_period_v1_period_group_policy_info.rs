/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodGroupPeriodV1PeriodGroupPolicyInfo : GroupPolicyInfo represents the high-level on-chain information for a group policy.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodGroupPeriodV1PeriodGroupPolicyInfo {
    /// address is the account address of group policy.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// group_id is the unique ID of the group.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// admin is the account address of the group admin.
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<String>,
    /// metadata is any arbitrary metadata to attached to the group policy.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// version is used to track changes to a group's GroupPolicyInfo structure that would create a different result on a running proposal.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "decisionPolicy", skip_serializing_if = "Option::is_none")]
    pub decision_policy: Option<crate::models::GooglePeriodProtobufPeriodAny>,
    /// created_at is a timestamp specifying when a group policy was created.
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl CosmosPeriodGroupPeriodV1PeriodGroupPolicyInfo {
    /// GroupPolicyInfo represents the high-level on-chain information for a group policy.
    pub fn new() -> CosmosPeriodGroupPeriodV1PeriodGroupPolicyInfo {
        CosmosPeriodGroupPeriodV1PeriodGroupPolicyInfo {
            address: None,
            group_id: None,
            admin: None,
            metadata: None,
            version: None,
            decision_policy: None,
            created_at: None,
        }
    }
}


