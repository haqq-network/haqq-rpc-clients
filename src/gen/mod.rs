// @generated
#[cfg(feature = "amino")]
// @@protoc_insertion_point(attribute:amino)
pub mod amino {
    include!("amino.rs");
    // @@protoc_insertion_point(amino)
}
pub mod capability {
    #[cfg(feature = "capability_v1")]
    // @@protoc_insertion_point(attribute:capability.v1)
    pub mod v1 {
        include!("capability.v1.rs");
        // @@protoc_insertion_point(capability.v1)
    }
}
pub mod cosmos {
    pub mod app {
        pub mod runtime {
            #[cfg(feature = "cosmos_app_runtime_v1alpha1")]
            // @@protoc_insertion_point(attribute:cosmos.app.runtime.v1alpha1)
            pub mod v1alpha1 {
                include!("cosmos.app.runtime.v1alpha1.rs");
                // @@protoc_insertion_point(cosmos.app.runtime.v1alpha1)
            }
        }
        #[cfg(feature = "cosmos_app_v1alpha1")]
        // @@protoc_insertion_point(attribute:cosmos.app.v1alpha1)
        pub mod v1alpha1 {
            include!("cosmos.app.v1alpha1.rs");
            // @@protoc_insertion_point(cosmos.app.v1alpha1)
        }
    }
    pub mod auth {
        pub mod module {
            #[cfg(feature = "cosmos_auth_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.auth.module.v1)
            pub mod v1 {
                include!("cosmos.auth.module.v1.rs");
                // @@protoc_insertion_point(cosmos.auth.module.v1)
            }
        }
        #[cfg(feature = "cosmos_auth_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.auth.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.auth.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.auth.v1beta1)
        }
    }
    pub mod authz {
        pub mod module {
            #[cfg(feature = "cosmos_authz_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.authz.module.v1)
            pub mod v1 {
                include!("cosmos.authz.module.v1.rs");
                // @@protoc_insertion_point(cosmos.authz.module.v1)
            }
        }
        #[cfg(feature = "cosmos_authz_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.authz.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.authz.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.authz.v1beta1)
        }
    }
    pub mod autocli {
        #[cfg(feature = "cosmos_autocli_v1")]
        // @@protoc_insertion_point(attribute:cosmos.autocli.v1)
        pub mod v1 {
            include!("cosmos.autocli.v1.rs");
            // @@protoc_insertion_point(cosmos.autocli.v1)
        }
    }
    pub mod bank {
        pub mod module {
            #[cfg(feature = "cosmos_bank_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.bank.module.v1)
            pub mod v1 {
                include!("cosmos.bank.module.v1.rs");
                // @@protoc_insertion_point(cosmos.bank.module.v1)
            }
        }
        #[cfg(feature = "cosmos_bank_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.bank.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.bank.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.bank.v1beta1)
        }
    }
    pub mod base {
        pub mod abci {
            #[cfg(feature = "cosmos_base_abci_v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.base.abci.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.abci.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.abci.v1beta1)
            }
        }
        pub mod kv {
            #[cfg(feature = "cosmos_base_kv_v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.base.kv.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.kv.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.kv.v1beta1)
            }
        }
        pub mod node {
            #[cfg(feature = "cosmos_base_node_v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.base.node.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.node.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.node.v1beta1)
            }
        }
        pub mod query {
            #[cfg(feature = "cosmos_base_query_v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.base.query.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.query.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.query.v1beta1)
            }
        }
        pub mod reflection {
            #[cfg(feature = "cosmos_base_reflection_v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.base.reflection.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.reflection.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.reflection.v1beta1)
            }
            #[cfg(feature = "cosmos_base_reflection_v2alpha1")]
            // @@protoc_insertion_point(attribute:cosmos.base.reflection.v2alpha1)
            pub mod v2alpha1 {
                include!("cosmos.base.reflection.v2alpha1.rs");
                // @@protoc_insertion_point(cosmos.base.reflection.v2alpha1)
            }
        }
        pub mod snapshots {
            #[cfg(feature = "cosmos_base_snapshots_v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.base.snapshots.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.snapshots.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.snapshots.v1beta1)
            }
        }
        pub mod store {
            #[cfg(feature = "cosmos_base_store_v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.base.store.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.store.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.store.v1beta1)
            }
        }
        pub mod tendermint {
            #[cfg(feature = "cosmos_base_tendermint_v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.base.tendermint.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.tendermint.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.tendermint.v1beta1)
            }
        }
        #[cfg(feature = "cosmos_base_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.base.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.base.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.base.v1beta1)
        }
    }
    pub mod capability {
        pub mod module {
            #[cfg(feature = "cosmos_capability_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.capability.module.v1)
            pub mod v1 {
                include!("cosmos.capability.module.v1.rs");
                // @@protoc_insertion_point(cosmos.capability.module.v1)
            }
        }
        #[cfg(feature = "cosmos_capability_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.capability.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.capability.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.capability.v1beta1)
        }
    }
    pub mod consensus {
        pub mod module {
            #[cfg(feature = "cosmos_consensus_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.consensus.module.v1)
            pub mod v1 {
                include!("cosmos.consensus.module.v1.rs");
                // @@protoc_insertion_point(cosmos.consensus.module.v1)
            }
        }
        #[cfg(feature = "cosmos_consensus_v1")]
        // @@protoc_insertion_point(attribute:cosmos.consensus.v1)
        pub mod v1 {
            include!("cosmos.consensus.v1.rs");
            // @@protoc_insertion_point(cosmos.consensus.v1)
        }
    }
    pub mod crisis {
        pub mod module {
            #[cfg(feature = "cosmos_crisis_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.crisis.module.v1)
            pub mod v1 {
                include!("cosmos.crisis.module.v1.rs");
                // @@protoc_insertion_point(cosmos.crisis.module.v1)
            }
        }
        #[cfg(feature = "cosmos_crisis_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.crisis.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.crisis.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.crisis.v1beta1)
        }
    }
    pub mod crypto {
        #[cfg(feature = "cosmos_crypto_ed25519")]
        // @@protoc_insertion_point(attribute:cosmos.crypto.ed25519)
        pub mod ed25519 {
            include!("cosmos.crypto.ed25519.rs");
            // @@protoc_insertion_point(cosmos.crypto.ed25519)
        }
        pub mod hd {
            #[cfg(feature = "cosmos_crypto_hd_v1")]
            // @@protoc_insertion_point(attribute:cosmos.crypto.hd.v1)
            pub mod v1 {
                include!("cosmos.crypto.hd.v1.rs");
                // @@protoc_insertion_point(cosmos.crypto.hd.v1)
            }
        }
        pub mod keyring {
            #[cfg(feature = "cosmos_crypto_keyring_v1")]
            // @@protoc_insertion_point(attribute:cosmos.crypto.keyring.v1)
            pub mod v1 {
                include!("cosmos.crypto.keyring.v1.rs");
                // @@protoc_insertion_point(cosmos.crypto.keyring.v1)
            }
        }
        #[cfg(feature = "cosmos_crypto_multisig")]
        // @@protoc_insertion_point(attribute:cosmos.crypto.multisig)
        pub mod multisig {
            include!("cosmos.crypto.multisig.rs");
            // @@protoc_insertion_point(cosmos.crypto.multisig)
            #[cfg(feature = "cosmos_crypto_multisig_v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.crypto.multisig.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.crypto.multisig.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.crypto.multisig.v1beta1)
            }
        }
        #[cfg(feature = "cosmos_crypto_secp256k1")]
        // @@protoc_insertion_point(attribute:cosmos.crypto.secp256k1)
        pub mod secp256k1 {
            include!("cosmos.crypto.secp256k1.rs");
            // @@protoc_insertion_point(cosmos.crypto.secp256k1)
        }
        #[cfg(feature = "cosmos_crypto_secp256r1")]
        // @@protoc_insertion_point(attribute:cosmos.crypto.secp256r1)
        pub mod secp256r1 {
            include!("cosmos.crypto.secp256r1.rs");
            // @@protoc_insertion_point(cosmos.crypto.secp256r1)
        }
    }
    pub mod distribution {
        pub mod module {
            #[cfg(feature = "cosmos_distribution_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.distribution.module.v1)
            pub mod v1 {
                include!("cosmos.distribution.module.v1.rs");
                // @@protoc_insertion_point(cosmos.distribution.module.v1)
            }
        }
        #[cfg(feature = "cosmos_distribution_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.distribution.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.distribution.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.distribution.v1beta1)
        }
    }
    pub mod evidence {
        pub mod module {
            #[cfg(feature = "cosmos_evidence_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.evidence.module.v1)
            pub mod v1 {
                include!("cosmos.evidence.module.v1.rs");
                // @@protoc_insertion_point(cosmos.evidence.module.v1)
            }
        }
        #[cfg(feature = "cosmos_evidence_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.evidence.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.evidence.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.evidence.v1beta1)
        }
    }
    pub mod feegrant {
        pub mod module {
            #[cfg(feature = "cosmos_feegrant_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.feegrant.module.v1)
            pub mod v1 {
                include!("cosmos.feegrant.module.v1.rs");
                // @@protoc_insertion_point(cosmos.feegrant.module.v1)
            }
        }
        #[cfg(feature = "cosmos_feegrant_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.feegrant.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.feegrant.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.feegrant.v1beta1)
        }
    }
    pub mod genutil {
        pub mod module {
            #[cfg(feature = "cosmos_genutil_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.genutil.module.v1)
            pub mod v1 {
                include!("cosmos.genutil.module.v1.rs");
                // @@protoc_insertion_point(cosmos.genutil.module.v1)
            }
        }
        #[cfg(feature = "cosmos_genutil_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.genutil.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.genutil.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.genutil.v1beta1)
        }
    }
    pub mod gov {
        pub mod module {
            #[cfg(feature = "cosmos_gov_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.gov.module.v1)
            pub mod v1 {
                include!("cosmos.gov.module.v1.rs");
                // @@protoc_insertion_point(cosmos.gov.module.v1)
            }
        }
        #[cfg(feature = "cosmos_gov_v1")]
        // @@protoc_insertion_point(attribute:cosmos.gov.v1)
        pub mod v1 {
            include!("cosmos.gov.v1.rs");
            // @@protoc_insertion_point(cosmos.gov.v1)
        }
        #[cfg(feature = "cosmos_gov_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.gov.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.gov.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.gov.v1beta1)
        }
    }
    pub mod group {
        pub mod module {
            #[cfg(feature = "cosmos_group_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.group.module.v1)
            pub mod v1 {
                include!("cosmos.group.module.v1.rs");
                // @@protoc_insertion_point(cosmos.group.module.v1)
            }
        }
        #[cfg(feature = "cosmos_group_v1")]
        // @@protoc_insertion_point(attribute:cosmos.group.v1)
        pub mod v1 {
            include!("cosmos.group.v1.rs");
            // @@protoc_insertion_point(cosmos.group.v1)
        }
    }
    pub mod ics23 {
        #[cfg(feature = "cosmos_ics23_v1")]
        // @@protoc_insertion_point(attribute:cosmos.ics23.v1)
        pub mod v1 {
            include!("cosmos.ics23.v1.rs");
            // @@protoc_insertion_point(cosmos.ics23.v1)
        }
    }
    pub mod mint {
        pub mod module {
            #[cfg(feature = "cosmos_mint_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.mint.module.v1)
            pub mod v1 {
                include!("cosmos.mint.module.v1.rs");
                // @@protoc_insertion_point(cosmos.mint.module.v1)
            }
        }
        #[cfg(feature = "cosmos_mint_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.mint.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.mint.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.mint.v1beta1)
        }
    }
    pub mod msg {
        #[cfg(feature = "cosmos_msg_v1")]
        // @@protoc_insertion_point(attribute:cosmos.msg.v1)
        pub mod v1 {
            include!("cosmos.msg.v1.rs");
            // @@protoc_insertion_point(cosmos.msg.v1)
        }
    }
    pub mod orm {
        pub mod module {
            #[cfg(feature = "cosmos_orm_module_v1alpha1")]
            // @@protoc_insertion_point(attribute:cosmos.orm.module.v1alpha1)
            pub mod v1alpha1 {
                include!("cosmos.orm.module.v1alpha1.rs");
                // @@protoc_insertion_point(cosmos.orm.module.v1alpha1)
            }
        }
        pub mod query {
            #[cfg(feature = "cosmos_orm_query_v1alpha1")]
            // @@protoc_insertion_point(attribute:cosmos.orm.query.v1alpha1)
            pub mod v1alpha1 {
                include!("cosmos.orm.query.v1alpha1.rs");
                // @@protoc_insertion_point(cosmos.orm.query.v1alpha1)
            }
        }
        #[cfg(feature = "cosmos_orm_v1")]
        // @@protoc_insertion_point(attribute:cosmos.orm.v1)
        pub mod v1 {
            include!("cosmos.orm.v1.rs");
            // @@protoc_insertion_point(cosmos.orm.v1)
        }
        #[cfg(feature = "cosmos_orm_v1alpha1")]
        // @@protoc_insertion_point(attribute:cosmos.orm.v1alpha1)
        pub mod v1alpha1 {
            include!("cosmos.orm.v1alpha1.rs");
            // @@protoc_insertion_point(cosmos.orm.v1alpha1)
        }
    }
    pub mod params {
        pub mod module {
            #[cfg(feature = "cosmos_params_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.params.module.v1)
            pub mod v1 {
                include!("cosmos.params.module.v1.rs");
                // @@protoc_insertion_point(cosmos.params.module.v1)
            }
        }
        #[cfg(feature = "cosmos_params_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.params.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.params.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.params.v1beta1)
        }
    }
    pub mod query {
        #[cfg(feature = "cosmos_query_v1")]
        // @@protoc_insertion_point(attribute:cosmos.query.v1)
        pub mod v1 {
            include!("cosmos.query.v1.rs");
            // @@protoc_insertion_point(cosmos.query.v1)
        }
    }
    pub mod reflection {
        #[cfg(feature = "cosmos_reflection_v1")]
        // @@protoc_insertion_point(attribute:cosmos.reflection.v1)
        pub mod v1 {
            include!("cosmos.reflection.v1.rs");
            // @@protoc_insertion_point(cosmos.reflection.v1)
        }
    }
    pub mod slashing {
        pub mod module {
            #[cfg(feature = "cosmos_slashing_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.slashing.module.v1)
            pub mod v1 {
                include!("cosmos.slashing.module.v1.rs");
                // @@protoc_insertion_point(cosmos.slashing.module.v1)
            }
        }
        #[cfg(feature = "cosmos_slashing_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.slashing.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.slashing.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.slashing.v1beta1)
        }
    }
    pub mod staking {
        pub mod module {
            #[cfg(feature = "cosmos_staking_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.staking.module.v1)
            pub mod v1 {
                include!("cosmos.staking.module.v1.rs");
                // @@protoc_insertion_point(cosmos.staking.module.v1)
            }
        }
        #[cfg(feature = "cosmos_staking_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.staking.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.staking.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.staking.v1beta1)
        }
    }
    pub mod tx {
        pub mod config {
            #[cfg(feature = "cosmos_tx_config_v1")]
            // @@protoc_insertion_point(attribute:cosmos.tx.config.v1)
            pub mod v1 {
                include!("cosmos.tx.config.v1.rs");
                // @@protoc_insertion_point(cosmos.tx.config.v1)
            }
        }
        pub mod signing {
            #[cfg(feature = "cosmos_tx_signing_v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.tx.signing.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.tx.signing.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.tx.signing.v1beta1)
            }
        }
        #[cfg(feature = "cosmos_tx_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.tx.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.tx.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.tx.v1beta1)
        }
    }
    pub mod upgrade {
        pub mod module {
            #[cfg(feature = "cosmos_upgrade_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.upgrade.module.v1)
            pub mod v1 {
                include!("cosmos.upgrade.module.v1.rs");
                // @@protoc_insertion_point(cosmos.upgrade.module.v1)
            }
        }
        #[cfg(feature = "cosmos_upgrade_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.upgrade.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.upgrade.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.upgrade.v1beta1)
        }
    }
    pub mod vesting {
        pub mod module {
            #[cfg(feature = "cosmos_vesting_module_v1")]
            // @@protoc_insertion_point(attribute:cosmos.vesting.module.v1)
            pub mod v1 {
                include!("cosmos.vesting.module.v1.rs");
                // @@protoc_insertion_point(cosmos.vesting.module.v1)
            }
        }
        #[cfg(feature = "cosmos_vesting_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.vesting.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.vesting.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.vesting.v1beta1)
        }
    }
}
#[cfg(feature = "cosmos_proto")]
// @@protoc_insertion_point(attribute:cosmos_proto)
pub mod cosmos_proto {
    include!("cosmos_proto.rs");
    // @@protoc_insertion_point(cosmos_proto)
}
pub mod ethermint {
    pub mod crypto {
        pub mod v1 {
            #[cfg(feature = "ethermint_crypto_v1_ethsecp256k1")]
            // @@protoc_insertion_point(attribute:ethermint.crypto.v1.ethsecp256k1)
            pub mod ethsecp256k1 {
                include!("ethermint.crypto.v1.ethsecp256k1.rs");
                // @@protoc_insertion_point(ethermint.crypto.v1.ethsecp256k1)
            }
        }
    }
    pub mod evm {
        #[cfg(feature = "ethermint_evm_v1")]
        // @@protoc_insertion_point(attribute:ethermint.evm.v1)
        pub mod v1 {
            include!("ethermint.evm.v1.rs");
            // @@protoc_insertion_point(ethermint.evm.v1)
        }
    }
    pub mod feemarket {
        #[cfg(feature = "ethermint_feemarket_v1")]
        // @@protoc_insertion_point(attribute:ethermint.feemarket.v1)
        pub mod v1 {
            include!("ethermint.feemarket.v1.rs");
            // @@protoc_insertion_point(ethermint.feemarket.v1)
        }
    }
    pub mod types {
        #[cfg(feature = "ethermint_types_v1")]
        // @@protoc_insertion_point(attribute:ethermint.types.v1)
        pub mod v1 {
            include!("ethermint.types.v1.rs");
            // @@protoc_insertion_point(ethermint.types.v1)
        }
    }
}
pub mod evmos {
    pub mod epochs {
        #[cfg(feature = "evmos_epochs_v1")]
        // @@protoc_insertion_point(attribute:evmos.epochs.v1)
        pub mod v1 {
            include!("evmos.epochs.v1.rs");
            // @@protoc_insertion_point(evmos.epochs.v1)
        }
    }
    pub mod erc20 {
        #[cfg(feature = "evmos_erc20_v1")]
        // @@protoc_insertion_point(attribute:evmos.erc20.v1)
        pub mod v1 {
            include!("evmos.erc20.v1.rs");
            // @@protoc_insertion_point(evmos.erc20.v1)
        }
    }
}
pub mod haqq {
    pub mod coinomics {
        #[cfg(feature = "haqq_coinomics_v1")]
        // @@protoc_insertion_point(attribute:haqq.coinomics.v1)
        pub mod v1 {
            include!("haqq.coinomics.v1.rs");
            // @@protoc_insertion_point(haqq.coinomics.v1)
        }
    }
    pub mod dao {
        pub mod module {
            #[cfg(feature = "haqq_dao_module_v1")]
            // @@protoc_insertion_point(attribute:haqq.dao.module.v1)
            pub mod v1 {
                include!("haqq.dao.module.v1.rs");
                // @@protoc_insertion_point(haqq.dao.module.v1)
            }
        }
        #[cfg(feature = "haqq_dao_v1")]
        // @@protoc_insertion_point(attribute:haqq.dao.v1)
        pub mod v1 {
            include!("haqq.dao.v1.rs");
            // @@protoc_insertion_point(haqq.dao.v1)
        }
    }
    pub mod liquidvesting {
        #[cfg(feature = "haqq_liquidvesting_v1")]
        // @@protoc_insertion_point(attribute:haqq.liquidvesting.v1)
        pub mod v1 {
            include!("haqq.liquidvesting.v1.rs");
            // @@protoc_insertion_point(haqq.liquidvesting.v1)
        }
    }
    pub mod vesting {
        #[cfg(feature = "haqq_vesting_v1")]
        // @@protoc_insertion_point(attribute:haqq.vesting.v1)
        pub mod v1 {
            include!("haqq.vesting.v1.rs");
            // @@protoc_insertion_point(haqq.vesting.v1)
        }
    }
}
pub mod ibc {
    pub mod applications {
        pub mod fee {
            #[cfg(feature = "ibc_applications_fee_v1")]
            // @@protoc_insertion_point(attribute:ibc.applications.fee.v1)
            pub mod v1 {
                include!("ibc.applications.fee.v1.rs");
                // @@protoc_insertion_point(ibc.applications.fee.v1)
            }
        }
        pub mod interchain_accounts {
            pub mod controller {
                #[cfg(feature = "ibc_applications_interchain_accounts_controller_v1")]
                // @@protoc_insertion_point(attribute:ibc.applications.interchain_accounts.controller.v1)
                pub mod v1 {
                    include!("ibc.applications.interchain_accounts.controller.v1.rs");
                    // @@protoc_insertion_point(ibc.applications.interchain_accounts.controller.v1)
                }
            }
            pub mod genesis {
                #[cfg(feature = "ibc_applications_interchain_accounts_genesis_v1")]
                // @@protoc_insertion_point(attribute:ibc.applications.interchain_accounts.genesis.v1)
                pub mod v1 {
                    include!("ibc.applications.interchain_accounts.genesis.v1.rs");
                    // @@protoc_insertion_point(ibc.applications.interchain_accounts.genesis.v1)
                }
            }
            pub mod host {
                #[cfg(feature = "ibc_applications_interchain_accounts_host_v1")]
                // @@protoc_insertion_point(attribute:ibc.applications.interchain_accounts.host.v1)
                pub mod v1 {
                    include!("ibc.applications.interchain_accounts.host.v1.rs");
                    // @@protoc_insertion_point(ibc.applications.interchain_accounts.host.v1)
                }
            }
            #[cfg(feature = "ibc_applications_interchain_accounts_v1")]
            // @@protoc_insertion_point(attribute:ibc.applications.interchain_accounts.v1)
            pub mod v1 {
                include!("ibc.applications.interchain_accounts.v1.rs");
                // @@protoc_insertion_point(ibc.applications.interchain_accounts.v1)
            }
        }
        pub mod transfer {
            #[cfg(feature = "ibc_applications_transfer_v1")]
            // @@protoc_insertion_point(attribute:ibc.applications.transfer.v1)
            pub mod v1 {
                include!("ibc.applications.transfer.v1.rs");
                // @@protoc_insertion_point(ibc.applications.transfer.v1)
            }
            #[cfg(feature = "ibc_applications_transfer_v2")]
            // @@protoc_insertion_point(attribute:ibc.applications.transfer.v2)
            pub mod v2 {
                include!("ibc.applications.transfer.v2.rs");
                // @@protoc_insertion_point(ibc.applications.transfer.v2)
            }
        }
    }
    pub mod core {
        pub mod channel {
            #[cfg(feature = "ibc_core_channel_v1")]
            // @@protoc_insertion_point(attribute:ibc.core.channel.v1)
            pub mod v1 {
                include!("ibc.core.channel.v1.rs");
                // @@protoc_insertion_point(ibc.core.channel.v1)
            }
        }
        pub mod client {
            #[cfg(feature = "ibc_core_client_v1")]
            // @@protoc_insertion_point(attribute:ibc.core.client.v1)
            pub mod v1 {
                include!("ibc.core.client.v1.rs");
                // @@protoc_insertion_point(ibc.core.client.v1)
            }
        }
        pub mod commitment {
            #[cfg(feature = "ibc_core_commitment_v1")]
            // @@protoc_insertion_point(attribute:ibc.core.commitment.v1)
            pub mod v1 {
                include!("ibc.core.commitment.v1.rs");
                // @@protoc_insertion_point(ibc.core.commitment.v1)
            }
        }
        pub mod connection {
            #[cfg(feature = "ibc_core_connection_v1")]
            // @@protoc_insertion_point(attribute:ibc.core.connection.v1)
            pub mod v1 {
                include!("ibc.core.connection.v1.rs");
                // @@protoc_insertion_point(ibc.core.connection.v1)
            }
        }
        pub mod types {
            #[cfg(feature = "ibc_core_types_v1")]
            // @@protoc_insertion_point(attribute:ibc.core.types.v1)
            pub mod v1 {
                include!("ibc.core.types.v1.rs");
                // @@protoc_insertion_point(ibc.core.types.v1)
            }
        }
    }
    pub mod lightclients {
        pub mod localhost {
            #[cfg(feature = "ibc_lightclients_localhost_v2")]
            // @@protoc_insertion_point(attribute:ibc.lightclients.localhost.v2)
            pub mod v2 {
                include!("ibc.lightclients.localhost.v2.rs");
                // @@protoc_insertion_point(ibc.lightclients.localhost.v2)
            }
        }
        pub mod solomachine {
            #[cfg(feature = "ibc_lightclients_solomachine_v2")]
            // @@protoc_insertion_point(attribute:ibc.lightclients.solomachine.v2)
            pub mod v2 {
                include!("ibc.lightclients.solomachine.v2.rs");
                // @@protoc_insertion_point(ibc.lightclients.solomachine.v2)
            }
            #[cfg(feature = "ibc_lightclients_solomachine_v3")]
            // @@protoc_insertion_point(attribute:ibc.lightclients.solomachine.v3)
            pub mod v3 {
                include!("ibc.lightclients.solomachine.v3.rs");
                // @@protoc_insertion_point(ibc.lightclients.solomachine.v3)
            }
        }
        pub mod tendermint {
            #[cfg(feature = "ibc_lightclients_tendermint_v1")]
            // @@protoc_insertion_point(attribute:ibc.lightclients.tendermint.v1)
            pub mod v1 {
                include!("ibc.lightclients.tendermint.v1.rs");
                // @@protoc_insertion_point(ibc.lightclients.tendermint.v1)
            }
        }
    }
}
pub mod tendermint {
    #[cfg(feature = "tendermint_abci")]
    // @@protoc_insertion_point(attribute:tendermint.abci)
    pub mod abci {
        include!("tendermint.abci.rs");
        // @@protoc_insertion_point(tendermint.abci)
    }
    #[cfg(feature = "tendermint_crypto")]
    // @@protoc_insertion_point(attribute:tendermint.crypto)
    pub mod crypto {
        include!("tendermint.crypto.rs");
        // @@protoc_insertion_point(tendermint.crypto)
    }
    pub mod libs {
        #[cfg(feature = "tendermint_libs_bits")]
        // @@protoc_insertion_point(attribute:tendermint.libs.bits)
        pub mod bits {
            include!("tendermint.libs.bits.rs");
            // @@protoc_insertion_point(tendermint.libs.bits)
        }
    }
    #[cfg(feature = "tendermint_p2p")]
    // @@protoc_insertion_point(attribute:tendermint.p2p)
    pub mod p2p {
        include!("tendermint.p2p.rs");
        // @@protoc_insertion_point(tendermint.p2p)
    }
    #[cfg(feature = "tendermint_types")]
    // @@protoc_insertion_point(attribute:tendermint.types)
    pub mod types {
        include!("tendermint.types.rs");
        // @@protoc_insertion_point(tendermint.types)
    }
    #[cfg(feature = "tendermint_version")]
    // @@protoc_insertion_point(attribute:tendermint.version)
    pub mod version {
        include!("tendermint.version.rs");
        // @@protoc_insertion_point(tendermint.version)
    }
}