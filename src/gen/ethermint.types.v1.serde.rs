// @generated
impl serde::Serialize for EthAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_account.is_some() {
            len += 1;
        }
        if !self.code_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.types.v1.EthAccount", len)?;
        if let Some(v) = self.base_account.as_ref() {
            struct_ser.serialize_field("baseAccount", v)?;
        }
        if !self.code_hash.is_empty() {
            struct_ser.serialize_field("codeHash", &self.code_hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EthAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_account",
            "baseAccount",
            "code_hash",
            "codeHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseAccount,
            CodeHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "baseAccount" | "base_account" => Ok(GeneratedField::BaseAccount),
                            "codeHash" | "code_hash" => Ok(GeneratedField::CodeHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EthAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.types.v1.EthAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EthAccount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_account__ = None;
                let mut code_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseAccount => {
                            if base_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseAccount"));
                            }
                            base_account__ = map_.next_value()?;
                        }
                        GeneratedField::CodeHash => {
                            if code_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeHash"));
                            }
                            code_hash__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EthAccount {
                    base_account: base_account__,
                    code_hash: code_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.types.v1.EthAccount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtensionOptionDynamicFeeTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.max_priority_price.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.types.v1.ExtensionOptionDynamicFeeTx", len)?;
        if !self.max_priority_price.is_empty() {
            struct_ser.serialize_field("maxPriorityPrice", &self.max_priority_price)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtensionOptionDynamicFeeTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_priority_price",
            "maxPriorityPrice",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxPriorityPrice,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxPriorityPrice" | "max_priority_price" => Ok(GeneratedField::MaxPriorityPrice),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtensionOptionDynamicFeeTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.types.v1.ExtensionOptionDynamicFeeTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExtensionOptionDynamicFeeTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_priority_price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxPriorityPrice => {
                            if max_priority_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPriorityPrice"));
                            }
                            max_priority_price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExtensionOptionDynamicFeeTx {
                    max_priority_price: max_priority_price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.types.v1.ExtensionOptionDynamicFeeTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtensionOptionsWeb3Tx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.typed_data_chain_id != 0 {
            len += 1;
        }
        if !self.fee_payer.is_empty() {
            len += 1;
        }
        if !self.fee_payer_sig.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.types.v1.ExtensionOptionsWeb3Tx", len)?;
        if self.typed_data_chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("typedDataChainId", ToString::to_string(&self.typed_data_chain_id).as_str())?;
        }
        if !self.fee_payer.is_empty() {
            struct_ser.serialize_field("feePayer", &self.fee_payer)?;
        }
        if !self.fee_payer_sig.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("feePayerSig", pbjson::private::base64::encode(&self.fee_payer_sig).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtensionOptionsWeb3Tx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "typed_data_chain_id",
            "typedDataChainId",
            "fee_payer",
            "feePayer",
            "fee_payer_sig",
            "feePayerSig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypedDataChainId,
            FeePayer,
            FeePayerSig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "typedDataChainId" | "typed_data_chain_id" => Ok(GeneratedField::TypedDataChainId),
                            "feePayer" | "fee_payer" => Ok(GeneratedField::FeePayer),
                            "feePayerSig" | "fee_payer_sig" => Ok(GeneratedField::FeePayerSig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtensionOptionsWeb3Tx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.types.v1.ExtensionOptionsWeb3Tx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExtensionOptionsWeb3Tx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut typed_data_chain_id__ = None;
                let mut fee_payer__ = None;
                let mut fee_payer_sig__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TypedDataChainId => {
                            if typed_data_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedDataChainId"));
                            }
                            typed_data_chain_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FeePayer => {
                            if fee_payer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feePayer"));
                            }
                            fee_payer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeePayerSig => {
                            if fee_payer_sig__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feePayerSig"));
                            }
                            fee_payer_sig__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ExtensionOptionsWeb3Tx {
                    typed_data_chain_id: typed_data_chain_id__.unwrap_or_default(),
                    fee_payer: fee_payer__.unwrap_or_default(),
                    fee_payer_sig: fee_payer_sig__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.types.v1.ExtensionOptionsWeb3Tx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        if self.tx_index != 0 {
            len += 1;
        }
        if self.msg_index != 0 {
            len += 1;
        }
        if self.eth_tx_index != 0 {
            len += 1;
        }
        if self.failed {
            len += 1;
        }
        if self.gas_used != 0 {
            len += 1;
        }
        if self.cumulative_gas_used != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.types.v1.TxResult", len)?;
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if self.tx_index != 0 {
            struct_ser.serialize_field("txIndex", &self.tx_index)?;
        }
        if self.msg_index != 0 {
            struct_ser.serialize_field("msgIndex", &self.msg_index)?;
        }
        if self.eth_tx_index != 0 {
            struct_ser.serialize_field("ethTxIndex", &self.eth_tx_index)?;
        }
        if self.failed {
            struct_ser.serialize_field("failed", &self.failed)?;
        }
        if self.gas_used != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("gasUsed", ToString::to_string(&self.gas_used).as_str())?;
        }
        if self.cumulative_gas_used != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("cumulativeGasUsed", ToString::to_string(&self.cumulative_gas_used).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
            "tx_index",
            "txIndex",
            "msg_index",
            "msgIndex",
            "eth_tx_index",
            "ethTxIndex",
            "failed",
            "gas_used",
            "gasUsed",
            "cumulative_gas_used",
            "cumulativeGasUsed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
            TxIndex,
            MsgIndex,
            EthTxIndex,
            Failed,
            GasUsed,
            CumulativeGasUsed,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "height" => Ok(GeneratedField::Height),
                            "txIndex" | "tx_index" => Ok(GeneratedField::TxIndex),
                            "msgIndex" | "msg_index" => Ok(GeneratedField::MsgIndex),
                            "ethTxIndex" | "eth_tx_index" => Ok(GeneratedField::EthTxIndex),
                            "failed" => Ok(GeneratedField::Failed),
                            "gasUsed" | "gas_used" => Ok(GeneratedField::GasUsed),
                            "cumulativeGasUsed" | "cumulative_gas_used" => Ok(GeneratedField::CumulativeGasUsed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.types.v1.TxResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                let mut tx_index__ = None;
                let mut msg_index__ = None;
                let mut eth_tx_index__ = None;
                let mut failed__ = None;
                let mut gas_used__ = None;
                let mut cumulative_gas_used__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TxIndex => {
                            if tx_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txIndex"));
                            }
                            tx_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MsgIndex => {
                            if msg_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msgIndex"));
                            }
                            msg_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EthTxIndex => {
                            if eth_tx_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethTxIndex"));
                            }
                            eth_tx_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Failed => {
                            if failed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            failed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GasUsed => {
                            if gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasUsed"));
                            }
                            gas_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CumulativeGasUsed => {
                            if cumulative_gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cumulativeGasUsed"));
                            }
                            cumulative_gas_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TxResult {
                    height: height__.unwrap_or_default(),
                    tx_index: tx_index__.unwrap_or_default(),
                    msg_index: msg_index__.unwrap_or_default(),
                    eth_tx_index: eth_tx_index__.unwrap_or_default(),
                    failed: failed__.unwrap_or_default(),
                    gas_used: gas_used__.unwrap_or_default(),
                    cumulative_gas_used: cumulative_gas_used__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.types.v1.TxResult", FIELDS, GeneratedVisitor)
    }
}
