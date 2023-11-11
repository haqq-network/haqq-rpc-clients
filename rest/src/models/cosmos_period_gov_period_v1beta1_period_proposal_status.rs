/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodGovPeriodV1beta1PeriodProposalStatus : ProposalStatus enumerates the valid statuses of a proposal.   - PROPOSAL_STATUS_UNSPECIFIED: PROPOSAL_STATUS_UNSPECIFIED defines the default proposal status.  - PROPOSAL_STATUS_DEPOSIT_PERIOD: PROPOSAL_STATUS_DEPOSIT_PERIOD defines a proposal status during the deposit period.  - PROPOSAL_STATUS_VOTING_PERIOD: PROPOSAL_STATUS_VOTING_PERIOD defines a proposal status during the voting period.  - PROPOSAL_STATUS_PASSED: PROPOSAL_STATUS_PASSED defines a proposal status of a proposal that has passed.  - PROPOSAL_STATUS_REJECTED: PROPOSAL_STATUS_REJECTED defines a proposal status of a proposal that has been rejected.  - PROPOSAL_STATUS_FAILED: PROPOSAL_STATUS_FAILED defines a proposal status of a proposal that has failed.

/// ProposalStatus enumerates the valid statuses of a proposal.   - PROPOSAL_STATUS_UNSPECIFIED: PROPOSAL_STATUS_UNSPECIFIED defines the default proposal status.  - PROPOSAL_STATUS_DEPOSIT_PERIOD: PROPOSAL_STATUS_DEPOSIT_PERIOD defines a proposal status during the deposit period.  - PROPOSAL_STATUS_VOTING_PERIOD: PROPOSAL_STATUS_VOTING_PERIOD defines a proposal status during the voting period.  - PROPOSAL_STATUS_PASSED: PROPOSAL_STATUS_PASSED defines a proposal status of a proposal that has passed.  - PROPOSAL_STATUS_REJECTED: PROPOSAL_STATUS_REJECTED defines a proposal status of a proposal that has been rejected.  - PROPOSAL_STATUS_FAILED: PROPOSAL_STATUS_FAILED defines a proposal status of a proposal that has failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CosmosPeriodGovPeriodV1beta1PeriodProposalStatus {
    #[serde(rename = "PROPOSAL_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "PROPOSAL_STATUS_DEPOSIT_PERIOD")]
    DepositPeriod,
    #[serde(rename = "PROPOSAL_STATUS_VOTING_PERIOD")]
    VotingPeriod,
    #[serde(rename = "PROPOSAL_STATUS_PASSED")]
    Passed,
    #[serde(rename = "PROPOSAL_STATUS_REJECTED")]
    Rejected,
    #[serde(rename = "PROPOSAL_STATUS_FAILED")]
    Failed,

}

impl ToString for CosmosPeriodGovPeriodV1beta1PeriodProposalStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Unspecified => String::from("PROPOSAL_STATUS_UNSPECIFIED"),
            Self::DepositPeriod => String::from("PROPOSAL_STATUS_DEPOSIT_PERIOD"),
            Self::VotingPeriod => String::from("PROPOSAL_STATUS_VOTING_PERIOD"),
            Self::Passed => String::from("PROPOSAL_STATUS_PASSED"),
            Self::Rejected => String::from("PROPOSAL_STATUS_REJECTED"),
            Self::Failed => String::from("PROPOSAL_STATUS_FAILED"),
        }
    }
}

impl Default for CosmosPeriodGovPeriodV1beta1PeriodProposalStatus {
    fn default() -> CosmosPeriodGovPeriodV1beta1PeriodProposalStatus {
        Self::Unspecified
    }
}




