// @generated
pub mod cosmos {
    pub mod app {
        pub mod module {
            #[cfg(feature = "cosmos_app_module_v1alpha1")]
            // @@protoc_insertion_point(attribute:cosmos.app.module.v1alpha1)
            pub mod v1alpha1 {
                include!("cosmos.app.module.v1alpha1.rs");
                // @@protoc_insertion_point(cosmos.app.module.v1alpha1)
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
        #[cfg(feature = "cosmos_auth_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.auth.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.auth.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.auth.v1beta1)
        }
    }
    pub mod authz {
        #[cfg(feature = "cosmos_authz_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.authz.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.authz.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.authz.v1beta1)
        }
    }
    pub mod bank {
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
        #[cfg(feature = "cosmos_capability_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.capability.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.capability.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.capability.v1beta1)
        }
    }
    pub mod crisis {
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
        #[cfg(feature = "cosmos_distribution_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.distribution.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.distribution.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.distribution.v1beta1)
        }
    }
    pub mod evidence {
        #[cfg(feature = "cosmos_evidence_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.evidence.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.evidence.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.evidence.v1beta1)
        }
    }
    pub mod feegrant {
        #[cfg(feature = "cosmos_feegrant_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.feegrant.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.feegrant.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.feegrant.v1beta1)
        }
    }
    pub mod genutil {
        #[cfg(feature = "cosmos_genutil_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.genutil.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.genutil.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.genutil.v1beta1)
        }
    }
    pub mod gov {
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
        #[cfg(feature = "cosmos_group_v1")]
        // @@protoc_insertion_point(attribute:cosmos.group.v1)
        pub mod v1 {
            include!("cosmos.group.v1.rs");
            // @@protoc_insertion_point(cosmos.group.v1)
        }
    }
    pub mod mint {
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
        #[cfg(feature = "cosmos_params_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.params.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.params.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.params.v1beta1)
        }
    }
    pub mod slashing {
        #[cfg(feature = "cosmos_slashing_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.slashing.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.slashing.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.slashing.v1beta1)
        }
    }
    pub mod staking {
        #[cfg(feature = "cosmos_staking_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.staking.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.staking.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.staking.v1beta1)
        }
    }
    pub mod tx {
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
        #[cfg(feature = "cosmos_upgrade_v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.upgrade.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.upgrade.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.upgrade.v1beta1)
        }
    }
    pub mod vesting {
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
    pub mod vesting {
        #[cfg(feature = "haqq_vesting_v1")]
        // @@protoc_insertion_point(attribute:haqq.vesting.v1)
        pub mod v1 {
            include!("haqq.vesting.v1.rs");
            // @@protoc_insertion_point(haqq.vesting.v1)
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