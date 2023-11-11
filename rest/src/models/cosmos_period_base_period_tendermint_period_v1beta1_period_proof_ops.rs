/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodProofOps : ProofOps is Merkle proof defined by the list of ProofOps.  Note: This type is a duplicate of the ProofOps proto type defined in Tendermint.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodProofOps {
    #[serde(rename = "ops", skip_serializing_if = "Option::is_none")]
    pub ops: Option<Vec<crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodProofOp>>,
}

impl CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodProofOps {
    /// ProofOps is Merkle proof defined by the list of ProofOps.  Note: This type is a duplicate of the ProofOps proto type defined in Tendermint.
    pub fn new() -> CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodProofOps {
        CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodProofOps {
            ops: None,
        }
    }
}


