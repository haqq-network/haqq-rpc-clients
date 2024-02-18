// @generated
impl serde::Serialize for AccessListTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        if !self.gas_price.is_empty() {
            len += 1;
        }
        if self.gas != 0 {
            len += 1;
        }
        if !self.to.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.accesses.is_empty() {
            len += 1;
        }
        if !self.v.is_empty() {
            len += 1;
        }
        if !self.r.is_empty() {
            len += 1;
        }
        if !self.s.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.AccessListTx", len)?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.gas_price.is_empty() {
            struct_ser.serialize_field("gasPrice", &self.gas_price)?;
        }
        if self.gas != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("gas", ToString::to_string(&self.gas).as_str())?;
        }
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", &self.to)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if !self.accesses.is_empty() {
            struct_ser.serialize_field("accesses", &self.accesses)?;
        }
        if !self.v.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("v", pbjson::private::base64::encode(&self.v).as_str())?;
        }
        if !self.r.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("r", pbjson::private::base64::encode(&self.r).as_str())?;
        }
        if !self.s.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("s", pbjson::private::base64::encode(&self.s).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccessListTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "nonce",
            "gas_price",
            "gasPrice",
            "gas",
            "to",
            "value",
            "data",
            "accesses",
            "v",
            "r",
            "s",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            Nonce,
            GasPrice,
            Gas,
            To,
            Value,
            Data,
            Accesses,
            V,
            R,
            S,
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
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "gasPrice" | "gas_price" => Ok(GeneratedField::GasPrice),
                            "gas" => Ok(GeneratedField::Gas),
                            "to" => Ok(GeneratedField::To),
                            "value" => Ok(GeneratedField::Value),
                            "data" => Ok(GeneratedField::Data),
                            "accesses" => Ok(GeneratedField::Accesses),
                            "v" => Ok(GeneratedField::V),
                            "r" => Ok(GeneratedField::R),
                            "s" => Ok(GeneratedField::S),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessListTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.AccessListTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AccessListTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut nonce__ = None;
                let mut gas_price__ = None;
                let mut gas__ = None;
                let mut to__ = None;
                let mut value__ = None;
                let mut data__ = None;
                let mut accesses__ = None;
                let mut v__ = None;
                let mut r__ = None;
                let mut s__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GasPrice => {
                            if gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasPrice"));
                            }
                            gas_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Gas => {
                            if gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gas"));
                            }
                            gas__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Accesses => {
                            if accesses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accesses"));
                            }
                            accesses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::V => {
                            if v__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v"));
                            }
                            v__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::R => {
                            if r__.is_some() {
                                return Err(serde::de::Error::duplicate_field("r"));
                            }
                            r__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::S => {
                            if s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("s"));
                            }
                            s__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AccessListTx {
                    chain_id: chain_id__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    gas_price: gas_price__.unwrap_or_default(),
                    gas: gas__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    accesses: accesses__.unwrap_or_default(),
                    v: v__.unwrap_or_default(),
                    r: r__.unwrap_or_default(),
                    s: s__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.AccessListTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AccessTuple {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.storage_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.AccessTuple", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.storage_keys.is_empty() {
            struct_ser.serialize_field("storageKeys", &self.storage_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccessTuple {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "storage_keys",
            "storageKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            StorageKeys,
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
                            "address" => Ok(GeneratedField::Address),
                            "storageKeys" | "storage_keys" => Ok(GeneratedField::StorageKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessTuple;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.AccessTuple")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AccessTuple, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut storage_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorageKeys => {
                            if storage_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageKeys"));
                            }
                            storage_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AccessTuple {
                    address: address__.unwrap_or_default(),
                    storage_keys: storage_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.AccessTuple", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChainConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.homestead_block.is_empty() {
            len += 1;
        }
        if !self.dao_fork_block.is_empty() {
            len += 1;
        }
        if self.dao_fork_support {
            len += 1;
        }
        if !self.eip150_block.is_empty() {
            len += 1;
        }
        if !self.eip150_hash.is_empty() {
            len += 1;
        }
        if !self.eip155_block.is_empty() {
            len += 1;
        }
        if !self.eip158_block.is_empty() {
            len += 1;
        }
        if !self.byzantium_block.is_empty() {
            len += 1;
        }
        if !self.constantinople_block.is_empty() {
            len += 1;
        }
        if !self.petersburg_block.is_empty() {
            len += 1;
        }
        if !self.istanbul_block.is_empty() {
            len += 1;
        }
        if !self.muir_glacier_block.is_empty() {
            len += 1;
        }
        if !self.berlin_block.is_empty() {
            len += 1;
        }
        if !self.london_block.is_empty() {
            len += 1;
        }
        if !self.arrow_glacier_block.is_empty() {
            len += 1;
        }
        if !self.gray_glacier_block.is_empty() {
            len += 1;
        }
        if !self.merge_netsplit_block.is_empty() {
            len += 1;
        }
        if !self.shanghai_block.is_empty() {
            len += 1;
        }
        if !self.cancun_block.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.ChainConfig", len)?;
        if !self.homestead_block.is_empty() {
            struct_ser.serialize_field("homesteadBlock", &self.homestead_block)?;
        }
        if !self.dao_fork_block.is_empty() {
            struct_ser.serialize_field("daoForkBlock", &self.dao_fork_block)?;
        }
        if self.dao_fork_support {
            struct_ser.serialize_field("daoForkSupport", &self.dao_fork_support)?;
        }
        if !self.eip150_block.is_empty() {
            struct_ser.serialize_field("eip150Block", &self.eip150_block)?;
        }
        if !self.eip150_hash.is_empty() {
            struct_ser.serialize_field("eip150Hash", &self.eip150_hash)?;
        }
        if !self.eip155_block.is_empty() {
            struct_ser.serialize_field("eip155Block", &self.eip155_block)?;
        }
        if !self.eip158_block.is_empty() {
            struct_ser.serialize_field("eip158Block", &self.eip158_block)?;
        }
        if !self.byzantium_block.is_empty() {
            struct_ser.serialize_field("byzantiumBlock", &self.byzantium_block)?;
        }
        if !self.constantinople_block.is_empty() {
            struct_ser.serialize_field("constantinopleBlock", &self.constantinople_block)?;
        }
        if !self.petersburg_block.is_empty() {
            struct_ser.serialize_field("petersburgBlock", &self.petersburg_block)?;
        }
        if !self.istanbul_block.is_empty() {
            struct_ser.serialize_field("istanbulBlock", &self.istanbul_block)?;
        }
        if !self.muir_glacier_block.is_empty() {
            struct_ser.serialize_field("muirGlacierBlock", &self.muir_glacier_block)?;
        }
        if !self.berlin_block.is_empty() {
            struct_ser.serialize_field("berlinBlock", &self.berlin_block)?;
        }
        if !self.london_block.is_empty() {
            struct_ser.serialize_field("londonBlock", &self.london_block)?;
        }
        if !self.arrow_glacier_block.is_empty() {
            struct_ser.serialize_field("arrowGlacierBlock", &self.arrow_glacier_block)?;
        }
        if !self.gray_glacier_block.is_empty() {
            struct_ser.serialize_field("grayGlacierBlock", &self.gray_glacier_block)?;
        }
        if !self.merge_netsplit_block.is_empty() {
            struct_ser.serialize_field("mergeNetsplitBlock", &self.merge_netsplit_block)?;
        }
        if !self.shanghai_block.is_empty() {
            struct_ser.serialize_field("shanghaiBlock", &self.shanghai_block)?;
        }
        if !self.cancun_block.is_empty() {
            struct_ser.serialize_field("cancunBlock", &self.cancun_block)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "homestead_block",
            "homesteadBlock",
            "dao_fork_block",
            "daoForkBlock",
            "dao_fork_support",
            "daoForkSupport",
            "eip150_block",
            "eip150Block",
            "eip150_hash",
            "eip150Hash",
            "eip155_block",
            "eip155Block",
            "eip158_block",
            "eip158Block",
            "byzantium_block",
            "byzantiumBlock",
            "constantinople_block",
            "constantinopleBlock",
            "petersburg_block",
            "petersburgBlock",
            "istanbul_block",
            "istanbulBlock",
            "muir_glacier_block",
            "muirGlacierBlock",
            "berlin_block",
            "berlinBlock",
            "london_block",
            "londonBlock",
            "arrow_glacier_block",
            "arrowGlacierBlock",
            "gray_glacier_block",
            "grayGlacierBlock",
            "merge_netsplit_block",
            "mergeNetsplitBlock",
            "shanghai_block",
            "shanghaiBlock",
            "cancun_block",
            "cancunBlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HomesteadBlock,
            DaoForkBlock,
            DaoForkSupport,
            Eip150Block,
            Eip150Hash,
            Eip155Block,
            Eip158Block,
            ByzantiumBlock,
            ConstantinopleBlock,
            PetersburgBlock,
            IstanbulBlock,
            MuirGlacierBlock,
            BerlinBlock,
            LondonBlock,
            ArrowGlacierBlock,
            GrayGlacierBlock,
            MergeNetsplitBlock,
            ShanghaiBlock,
            CancunBlock,
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
                            "homesteadBlock" | "homestead_block" => Ok(GeneratedField::HomesteadBlock),
                            "daoForkBlock" | "dao_fork_block" => Ok(GeneratedField::DaoForkBlock),
                            "daoForkSupport" | "dao_fork_support" => Ok(GeneratedField::DaoForkSupport),
                            "eip150Block" | "eip150_block" => Ok(GeneratedField::Eip150Block),
                            "eip150Hash" | "eip150_hash" => Ok(GeneratedField::Eip150Hash),
                            "eip155Block" | "eip155_block" => Ok(GeneratedField::Eip155Block),
                            "eip158Block" | "eip158_block" => Ok(GeneratedField::Eip158Block),
                            "byzantiumBlock" | "byzantium_block" => Ok(GeneratedField::ByzantiumBlock),
                            "constantinopleBlock" | "constantinople_block" => Ok(GeneratedField::ConstantinopleBlock),
                            "petersburgBlock" | "petersburg_block" => Ok(GeneratedField::PetersburgBlock),
                            "istanbulBlock" | "istanbul_block" => Ok(GeneratedField::IstanbulBlock),
                            "muirGlacierBlock" | "muir_glacier_block" => Ok(GeneratedField::MuirGlacierBlock),
                            "berlinBlock" | "berlin_block" => Ok(GeneratedField::BerlinBlock),
                            "londonBlock" | "london_block" => Ok(GeneratedField::LondonBlock),
                            "arrowGlacierBlock" | "arrow_glacier_block" => Ok(GeneratedField::ArrowGlacierBlock),
                            "grayGlacierBlock" | "gray_glacier_block" => Ok(GeneratedField::GrayGlacierBlock),
                            "mergeNetsplitBlock" | "merge_netsplit_block" => Ok(GeneratedField::MergeNetsplitBlock),
                            "shanghaiBlock" | "shanghai_block" => Ok(GeneratedField::ShanghaiBlock),
                            "cancunBlock" | "cancun_block" => Ok(GeneratedField::CancunBlock),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.ChainConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChainConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut homestead_block__ = None;
                let mut dao_fork_block__ = None;
                let mut dao_fork_support__ = None;
                let mut eip150_block__ = None;
                let mut eip150_hash__ = None;
                let mut eip155_block__ = None;
                let mut eip158_block__ = None;
                let mut byzantium_block__ = None;
                let mut constantinople_block__ = None;
                let mut petersburg_block__ = None;
                let mut istanbul_block__ = None;
                let mut muir_glacier_block__ = None;
                let mut berlin_block__ = None;
                let mut london_block__ = None;
                let mut arrow_glacier_block__ = None;
                let mut gray_glacier_block__ = None;
                let mut merge_netsplit_block__ = None;
                let mut shanghai_block__ = None;
                let mut cancun_block__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HomesteadBlock => {
                            if homestead_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("homesteadBlock"));
                            }
                            homestead_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DaoForkBlock => {
                            if dao_fork_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("daoForkBlock"));
                            }
                            dao_fork_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DaoForkSupport => {
                            if dao_fork_support__.is_some() {
                                return Err(serde::de::Error::duplicate_field("daoForkSupport"));
                            }
                            dao_fork_support__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Eip150Block => {
                            if eip150_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eip150Block"));
                            }
                            eip150_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Eip150Hash => {
                            if eip150_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eip150Hash"));
                            }
                            eip150_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Eip155Block => {
                            if eip155_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eip155Block"));
                            }
                            eip155_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Eip158Block => {
                            if eip158_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eip158Block"));
                            }
                            eip158_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ByzantiumBlock => {
                            if byzantium_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("byzantiumBlock"));
                            }
                            byzantium_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConstantinopleBlock => {
                            if constantinople_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constantinopleBlock"));
                            }
                            constantinople_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PetersburgBlock => {
                            if petersburg_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("petersburgBlock"));
                            }
                            petersburg_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IstanbulBlock => {
                            if istanbul_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("istanbulBlock"));
                            }
                            istanbul_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MuirGlacierBlock => {
                            if muir_glacier_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("muirGlacierBlock"));
                            }
                            muir_glacier_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BerlinBlock => {
                            if berlin_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("berlinBlock"));
                            }
                            berlin_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LondonBlock => {
                            if london_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("londonBlock"));
                            }
                            london_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ArrowGlacierBlock => {
                            if arrow_glacier_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arrowGlacierBlock"));
                            }
                            arrow_glacier_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GrayGlacierBlock => {
                            if gray_glacier_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grayGlacierBlock"));
                            }
                            gray_glacier_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MergeNetsplitBlock => {
                            if merge_netsplit_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mergeNetsplitBlock"));
                            }
                            merge_netsplit_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShanghaiBlock => {
                            if shanghai_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shanghaiBlock"));
                            }
                            shanghai_block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancunBlock => {
                            if cancun_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancunBlock"));
                            }
                            cancun_block__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ChainConfig {
                    homestead_block: homestead_block__.unwrap_or_default(),
                    dao_fork_block: dao_fork_block__.unwrap_or_default(),
                    dao_fork_support: dao_fork_support__.unwrap_or_default(),
                    eip150_block: eip150_block__.unwrap_or_default(),
                    eip150_hash: eip150_hash__.unwrap_or_default(),
                    eip155_block: eip155_block__.unwrap_or_default(),
                    eip158_block: eip158_block__.unwrap_or_default(),
                    byzantium_block: byzantium_block__.unwrap_or_default(),
                    constantinople_block: constantinople_block__.unwrap_or_default(),
                    petersburg_block: petersburg_block__.unwrap_or_default(),
                    istanbul_block: istanbul_block__.unwrap_or_default(),
                    muir_glacier_block: muir_glacier_block__.unwrap_or_default(),
                    berlin_block: berlin_block__.unwrap_or_default(),
                    london_block: london_block__.unwrap_or_default(),
                    arrow_glacier_block: arrow_glacier_block__.unwrap_or_default(),
                    gray_glacier_block: gray_glacier_block__.unwrap_or_default(),
                    merge_netsplit_block: merge_netsplit_block__.unwrap_or_default(),
                    shanghai_block: shanghai_block__.unwrap_or_default(),
                    cancun_block: cancun_block__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.ChainConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DynamicFeeTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        if !self.gas_tip_cap.is_empty() {
            len += 1;
        }
        if !self.gas_fee_cap.is_empty() {
            len += 1;
        }
        if self.gas != 0 {
            len += 1;
        }
        if !self.to.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.accesses.is_empty() {
            len += 1;
        }
        if !self.v.is_empty() {
            len += 1;
        }
        if !self.r.is_empty() {
            len += 1;
        }
        if !self.s.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.DynamicFeeTx", len)?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.gas_tip_cap.is_empty() {
            struct_ser.serialize_field("gasTipCap", &self.gas_tip_cap)?;
        }
        if !self.gas_fee_cap.is_empty() {
            struct_ser.serialize_field("gasFeeCap", &self.gas_fee_cap)?;
        }
        if self.gas != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("gas", ToString::to_string(&self.gas).as_str())?;
        }
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", &self.to)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if !self.accesses.is_empty() {
            struct_ser.serialize_field("accesses", &self.accesses)?;
        }
        if !self.v.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("v", pbjson::private::base64::encode(&self.v).as_str())?;
        }
        if !self.r.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("r", pbjson::private::base64::encode(&self.r).as_str())?;
        }
        if !self.s.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("s", pbjson::private::base64::encode(&self.s).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DynamicFeeTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "nonce",
            "gas_tip_cap",
            "gasTipCap",
            "gas_fee_cap",
            "gasFeeCap",
            "gas",
            "to",
            "value",
            "data",
            "accesses",
            "v",
            "r",
            "s",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            Nonce,
            GasTipCap,
            GasFeeCap,
            Gas,
            To,
            Value,
            Data,
            Accesses,
            V,
            R,
            S,
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
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "gasTipCap" | "gas_tip_cap" => Ok(GeneratedField::GasTipCap),
                            "gasFeeCap" | "gas_fee_cap" => Ok(GeneratedField::GasFeeCap),
                            "gas" => Ok(GeneratedField::Gas),
                            "to" => Ok(GeneratedField::To),
                            "value" => Ok(GeneratedField::Value),
                            "data" => Ok(GeneratedField::Data),
                            "accesses" => Ok(GeneratedField::Accesses),
                            "v" => Ok(GeneratedField::V),
                            "r" => Ok(GeneratedField::R),
                            "s" => Ok(GeneratedField::S),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DynamicFeeTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.DynamicFeeTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DynamicFeeTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut nonce__ = None;
                let mut gas_tip_cap__ = None;
                let mut gas_fee_cap__ = None;
                let mut gas__ = None;
                let mut to__ = None;
                let mut value__ = None;
                let mut data__ = None;
                let mut accesses__ = None;
                let mut v__ = None;
                let mut r__ = None;
                let mut s__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GasTipCap => {
                            if gas_tip_cap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasTipCap"));
                            }
                            gas_tip_cap__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GasFeeCap => {
                            if gas_fee_cap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasFeeCap"));
                            }
                            gas_fee_cap__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Gas => {
                            if gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gas"));
                            }
                            gas__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Accesses => {
                            if accesses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accesses"));
                            }
                            accesses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::V => {
                            if v__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v"));
                            }
                            v__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::R => {
                            if r__.is_some() {
                                return Err(serde::de::Error::duplicate_field("r"));
                            }
                            r__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::S => {
                            if s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("s"));
                            }
                            s__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DynamicFeeTx {
                    chain_id: chain_id__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    gas_tip_cap: gas_tip_cap__.unwrap_or_default(),
                    gas_fee_cap: gas_fee_cap__.unwrap_or_default(),
                    gas: gas__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    accesses: accesses__.unwrap_or_default(),
                    v: v__.unwrap_or_default(),
                    r: r__.unwrap_or_default(),
                    s: s__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.DynamicFeeTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EstimateGasResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.gas != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.EstimateGasResponse", len)?;
        if self.gas != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("gas", ToString::to_string(&self.gas).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EstimateGasResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "gas",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Gas,
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
                            "gas" => Ok(GeneratedField::Gas),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EstimateGasResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.EstimateGasResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EstimateGasResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut gas__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Gas => {
                            if gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gas"));
                            }
                            gas__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EstimateGasResponse {
                    gas: gas__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.EstimateGasResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EthCallRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.args.is_empty() {
            len += 1;
        }
        if self.gas_cap != 0 {
            len += 1;
        }
        if !self.proposer_address.is_empty() {
            len += 1;
        }
        if self.chain_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.EthCallRequest", len)?;
        if !self.args.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("args", pbjson::private::base64::encode(&self.args).as_str())?;
        }
        if self.gas_cap != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("gasCap", ToString::to_string(&self.gas_cap).as_str())?;
        }
        if !self.proposer_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("proposerAddress", pbjson::private::base64::encode(&self.proposer_address).as_str())?;
        }
        if self.chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EthCallRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "args",
            "gas_cap",
            "gasCap",
            "proposer_address",
            "proposerAddress",
            "chain_id",
            "chainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Args,
            GasCap,
            ProposerAddress,
            ChainId,
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
                            "args" => Ok(GeneratedField::Args),
                            "gasCap" | "gas_cap" => Ok(GeneratedField::GasCap),
                            "proposerAddress" | "proposer_address" => Ok(GeneratedField::ProposerAddress),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EthCallRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.EthCallRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EthCallRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut args__ = None;
                let mut gas_cap__ = None;
                let mut proposer_address__ = None;
                let mut chain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Args => {
                            if args__.is_some() {
                                return Err(serde::de::Error::duplicate_field("args"));
                            }
                            args__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GasCap => {
                            if gas_cap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasCap"));
                            }
                            gas_cap__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProposerAddress => {
                            if proposer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerAddress"));
                            }
                            proposer_address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EthCallRequest {
                    args: args__.unwrap_or_default(),
                    gas_cap: gas_cap__.unwrap_or_default(),
                    proposer_address: proposer_address__.unwrap_or_default(),
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.EthCallRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventBlockBloom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bloom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.EventBlockBloom", len)?;
        if !self.bloom.is_empty() {
            struct_ser.serialize_field("bloom", &self.bloom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventBlockBloom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bloom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bloom,
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
                            "bloom" => Ok(GeneratedField::Bloom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventBlockBloom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.EventBlockBloom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventBlockBloom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bloom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bloom => {
                            if bloom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bloom"));
                            }
                            bloom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventBlockBloom {
                    bloom: bloom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.EventBlockBloom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventEthereumTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.eth_hash.is_empty() {
            len += 1;
        }
        if !self.index.is_empty() {
            len += 1;
        }
        if !self.gas_used.is_empty() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.eth_tx_failed.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.EventEthereumTx", len)?;
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.eth_hash.is_empty() {
            struct_ser.serialize_field("ethHash", &self.eth_hash)?;
        }
        if !self.index.is_empty() {
            struct_ser.serialize_field("index", &self.index)?;
        }
        if !self.gas_used.is_empty() {
            struct_ser.serialize_field("gasUsed", &self.gas_used)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.eth_tx_failed.is_empty() {
            struct_ser.serialize_field("ethTxFailed", &self.eth_tx_failed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventEthereumTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "amount",
            "eth_hash",
            "ethHash",
            "index",
            "gas_used",
            "gasUsed",
            "hash",
            "recipient",
            "eth_tx_failed",
            "ethTxFailed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Amount,
            EthHash,
            Index,
            GasUsed,
            Hash,
            Recipient,
            EthTxFailed,
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
                            "amount" => Ok(GeneratedField::Amount),
                            "ethHash" | "eth_hash" => Ok(GeneratedField::EthHash),
                            "index" => Ok(GeneratedField::Index),
                            "gasUsed" | "gas_used" => Ok(GeneratedField::GasUsed),
                            "hash" => Ok(GeneratedField::Hash),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "ethTxFailed" | "eth_tx_failed" => Ok(GeneratedField::EthTxFailed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventEthereumTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.EventEthereumTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventEthereumTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                let mut eth_hash__ = None;
                let mut index__ = None;
                let mut gas_used__ = None;
                let mut hash__ = None;
                let mut recipient__ = None;
                let mut eth_tx_failed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EthHash => {
                            if eth_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethHash"));
                            }
                            eth_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GasUsed => {
                            if gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasUsed"));
                            }
                            gas_used__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EthTxFailed => {
                            if eth_tx_failed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethTxFailed"));
                            }
                            eth_tx_failed__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventEthereumTx {
                    amount: amount__.unwrap_or_default(),
                    eth_hash: eth_hash__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    gas_used: gas_used__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    eth_tx_failed: eth_tx_failed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.EventEthereumTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.module.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.tx_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.EventMessage", len)?;
        if !self.module.is_empty() {
            struct_ser.serialize_field("module", &self.module)?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.tx_type.is_empty() {
            struct_ser.serialize_field("txType", &self.tx_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "module",
            "sender",
            "tx_type",
            "txType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Module,
            Sender,
            TxType,
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
                            "module" => Ok(GeneratedField::Module),
                            "sender" => Ok(GeneratedField::Sender),
                            "txType" | "tx_type" => Ok(GeneratedField::TxType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.EventMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut module__ = None;
                let mut sender__ = None;
                let mut tx_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Module => {
                            if module__.is_some() {
                                return Err(serde::de::Error::duplicate_field("module"));
                            }
                            module__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TxType => {
                            if tx_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txType"));
                            }
                            tx_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventMessage {
                    module: module__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    tx_type: tx_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.EventMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventTxLog {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tx_logs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.EventTxLog", len)?;
        if !self.tx_logs.is_empty() {
            struct_ser.serialize_field("txLogs", &self.tx_logs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventTxLog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tx_logs",
            "txLogs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TxLogs,
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
                            "txLogs" | "tx_logs" => Ok(GeneratedField::TxLogs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventTxLog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.EventTxLog")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventTxLog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tx_logs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TxLogs => {
                            if tx_logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txLogs"));
                            }
                            tx_logs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventTxLog {
                    tx_logs: tx_logs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.EventTxLog", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtensionOptionsEthereumTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ethermint.evm.v1.ExtensionOptionsEthereumTx", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtensionOptionsEthereumTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtensionOptionsEthereumTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.ExtensionOptionsEthereumTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExtensionOptionsEthereumTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ExtensionOptionsEthereumTx {
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.ExtensionOptionsEthereumTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenesisAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.storage.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.GenesisAccount", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.storage.is_empty() {
            struct_ser.serialize_field("storage", &self.storage)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenesisAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "code",
            "storage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Code,
            Storage,
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
                            "address" => Ok(GeneratedField::Address),
                            "code" => Ok(GeneratedField::Code),
                            "storage" => Ok(GeneratedField::Storage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.GenesisAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisAccount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut code__ = None;
                let mut storage__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Storage => {
                            if storage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storage"));
                            }
                            storage__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisAccount {
                    address: address__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    storage: storage__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.GenesisAccount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.accounts.is_empty() {
            len += 1;
        }
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.GenesisState", len)?;
        if !self.accounts.is_empty() {
            struct_ser.serialize_field("accounts", &self.accounts)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenesisState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "accounts",
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Accounts,
            Params,
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
                            "accounts" => Ok(GeneratedField::Accounts),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut accounts__ = None;
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Accounts => {
                            if accounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accounts"));
                            }
                            accounts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GenesisState {
                    accounts: accounts__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.GenesisState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LegacyTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce != 0 {
            len += 1;
        }
        if !self.gas_price.is_empty() {
            len += 1;
        }
        if self.gas != 0 {
            len += 1;
        }
        if !self.to.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if !self.v.is_empty() {
            len += 1;
        }
        if !self.r.is_empty() {
            len += 1;
        }
        if !self.s.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.LegacyTx", len)?;
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.gas_price.is_empty() {
            struct_ser.serialize_field("gasPrice", &self.gas_price)?;
        }
        if self.gas != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("gas", ToString::to_string(&self.gas).as_str())?;
        }
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", &self.to)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if !self.v.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("v", pbjson::private::base64::encode(&self.v).as_str())?;
        }
        if !self.r.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("r", pbjson::private::base64::encode(&self.r).as_str())?;
        }
        if !self.s.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("s", pbjson::private::base64::encode(&self.s).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LegacyTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nonce",
            "gas_price",
            "gasPrice",
            "gas",
            "to",
            "value",
            "data",
            "v",
            "r",
            "s",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
            GasPrice,
            Gas,
            To,
            Value,
            Data,
            V,
            R,
            S,
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
                            "nonce" => Ok(GeneratedField::Nonce),
                            "gasPrice" | "gas_price" => Ok(GeneratedField::GasPrice),
                            "gas" => Ok(GeneratedField::Gas),
                            "to" => Ok(GeneratedField::To),
                            "value" => Ok(GeneratedField::Value),
                            "data" => Ok(GeneratedField::Data),
                            "v" => Ok(GeneratedField::V),
                            "r" => Ok(GeneratedField::R),
                            "s" => Ok(GeneratedField::S),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LegacyTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.LegacyTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LegacyTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                let mut gas_price__ = None;
                let mut gas__ = None;
                let mut to__ = None;
                let mut value__ = None;
                let mut data__ = None;
                let mut v__ = None;
                let mut r__ = None;
                let mut s__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GasPrice => {
                            if gas_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasPrice"));
                            }
                            gas_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Gas => {
                            if gas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gas"));
                            }
                            gas__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::V => {
                            if v__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v"));
                            }
                            v__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::R => {
                            if r__.is_some() {
                                return Err(serde::de::Error::duplicate_field("r"));
                            }
                            r__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::S => {
                            if s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("s"));
                            }
                            s__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LegacyTx {
                    nonce: nonce__.unwrap_or_default(),
                    gas_price: gas_price__.unwrap_or_default(),
                    gas: gas__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    v: v__.unwrap_or_default(),
                    r: r__.unwrap_or_default(),
                    s: s__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.LegacyTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Log {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.topics.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if self.block_number != 0 {
            len += 1;
        }
        if !self.tx_hash.is_empty() {
            len += 1;
        }
        if self.tx_index != 0 {
            len += 1;
        }
        if !self.block_hash.is_empty() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if self.removed {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.Log", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.topics.is_empty() {
            struct_ser.serialize_field("topics", &self.topics)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if self.block_number != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("blockNumber", ToString::to_string(&self.block_number).as_str())?;
        }
        if !self.tx_hash.is_empty() {
            struct_ser.serialize_field("txHash", &self.tx_hash)?;
        }
        if self.tx_index != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("txIndex", ToString::to_string(&self.tx_index).as_str())?;
        }
        if !self.block_hash.is_empty() {
            struct_ser.serialize_field("blockHash", &self.block_hash)?;
        }
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if self.removed {
            struct_ser.serialize_field("removed", &self.removed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Log {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "topics",
            "data",
            "block_number",
            "blockNumber",
            "tx_hash",
            "txHash",
            "tx_index",
            "txIndex",
            "block_hash",
            "blockHash",
            "index",
            "removed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Topics,
            Data,
            BlockNumber,
            TxHash,
            TxIndex,
            BlockHash,
            Index,
            Removed,
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
                            "address" => Ok(GeneratedField::Address),
                            "topics" => Ok(GeneratedField::Topics),
                            "data" => Ok(GeneratedField::Data),
                            "blockNumber" | "block_number" => Ok(GeneratedField::BlockNumber),
                            "txHash" | "tx_hash" => Ok(GeneratedField::TxHash),
                            "txIndex" | "tx_index" => Ok(GeneratedField::TxIndex),
                            "blockHash" | "block_hash" => Ok(GeneratedField::BlockHash),
                            "index" => Ok(GeneratedField::Index),
                            "removed" => Ok(GeneratedField::Removed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Log;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.Log")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Log, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut topics__ = None;
                let mut data__ = None;
                let mut block_number__ = None;
                let mut tx_hash__ = None;
                let mut tx_index__ = None;
                let mut block_hash__ = None;
                let mut index__ = None;
                let mut removed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Topics => {
                            if topics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("topics"));
                            }
                            topics__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockNumber => {
                            if block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockNumber"));
                            }
                            block_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TxHash => {
                            if tx_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHash"));
                            }
                            tx_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TxIndex => {
                            if tx_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txIndex"));
                            }
                            tx_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockHash => {
                            if block_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHash"));
                            }
                            block_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Removed => {
                            if removed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removed"));
                            }
                            removed__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Log {
                    address: address__.unwrap_or_default(),
                    topics: topics__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    block_number: block_number__.unwrap_or_default(),
                    tx_hash: tx_hash__.unwrap_or_default(),
                    tx_index: tx_index__.unwrap_or_default(),
                    block_hash: block_hash__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    removed: removed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.Log", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgEthereumTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        if self.size != 0. {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if !self.from.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.MsgEthereumTx", len)?;
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if self.size != 0. {
            struct_ser.serialize_field("size", &self.size)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgEthereumTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
            "size",
            "hash",
            "from",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
            Size,
            Hash,
            From,
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
                            "data" => Ok(GeneratedField::Data),
                            "size" => Ok(GeneratedField::Size),
                            "hash" => Ok(GeneratedField::Hash),
                            "from" => Ok(GeneratedField::From),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgEthereumTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.MsgEthereumTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgEthereumTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                let mut size__ = None;
                let mut hash__ = None;
                let mut from__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgEthereumTx {
                    data: data__,
                    size: size__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    from: from__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.MsgEthereumTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgEthereumTxResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if !self.logs.is_empty() {
            len += 1;
        }
        if !self.ret.is_empty() {
            len += 1;
        }
        if !self.vm_error.is_empty() {
            len += 1;
        }
        if self.gas_used != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.MsgEthereumTxResponse", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if !self.logs.is_empty() {
            struct_ser.serialize_field("logs", &self.logs)?;
        }
        if !self.ret.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("ret", pbjson::private::base64::encode(&self.ret).as_str())?;
        }
        if !self.vm_error.is_empty() {
            struct_ser.serialize_field("vmError", &self.vm_error)?;
        }
        if self.gas_used != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("gasUsed", ToString::to_string(&self.gas_used).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgEthereumTxResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "logs",
            "ret",
            "vm_error",
            "vmError",
            "gas_used",
            "gasUsed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Logs,
            Ret,
            VmError,
            GasUsed,
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
                            "hash" => Ok(GeneratedField::Hash),
                            "logs" => Ok(GeneratedField::Logs),
                            "ret" => Ok(GeneratedField::Ret),
                            "vmError" | "vm_error" => Ok(GeneratedField::VmError),
                            "gasUsed" | "gas_used" => Ok(GeneratedField::GasUsed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgEthereumTxResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.MsgEthereumTxResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgEthereumTxResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut logs__ = None;
                let mut ret__ = None;
                let mut vm_error__ = None;
                let mut gas_used__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Logs => {
                            if logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logs"));
                            }
                            logs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ret => {
                            if ret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ret"));
                            }
                            ret__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VmError => {
                            if vm_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vmError"));
                            }
                            vm_error__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GasUsed => {
                            if gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasUsed"));
                            }
                            gas_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MsgEthereumTxResponse {
                    hash: hash__.unwrap_or_default(),
                    logs: logs__.unwrap_or_default(),
                    ret: ret__.unwrap_or_default(),
                    vm_error: vm_error__.unwrap_or_default(),
                    gas_used: gas_used__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.MsgEthereumTxResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.MsgUpdateParams", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Params,
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
                            "authority" => Ok(GeneratedField::Authority),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.MsgUpdateParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateParams {
                    authority: authority__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.MsgUpdateParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ethermint.evm.v1.MsgUpdateParamsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.MsgUpdateParamsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateParamsResponse {
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.MsgUpdateParamsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.evm_denom.is_empty() {
            len += 1;
        }
        if self.enable_create {
            len += 1;
        }
        if self.enable_call {
            len += 1;
        }
        if !self.extra_eips.is_empty() {
            len += 1;
        }
        if self.chain_config.is_some() {
            len += 1;
        }
        if self.allow_unprotected_txs {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.Params", len)?;
        if !self.evm_denom.is_empty() {
            struct_ser.serialize_field("evmDenom", &self.evm_denom)?;
        }
        if self.enable_create {
            struct_ser.serialize_field("enableCreate", &self.enable_create)?;
        }
        if self.enable_call {
            struct_ser.serialize_field("enableCall", &self.enable_call)?;
        }
        if !self.extra_eips.is_empty() {
            struct_ser.serialize_field("extraEips", &self.extra_eips.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.chain_config.as_ref() {
            struct_ser.serialize_field("chainConfig", v)?;
        }
        if self.allow_unprotected_txs {
            struct_ser.serialize_field("allowUnprotectedTxs", &self.allow_unprotected_txs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "evm_denom",
            "evmDenom",
            "enable_create",
            "enableCreate",
            "enable_call",
            "enableCall",
            "extra_eips",
            "extraEips",
            "chain_config",
            "chainConfig",
            "allow_unprotected_txs",
            "allowUnprotectedTxs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EvmDenom,
            EnableCreate,
            EnableCall,
            ExtraEips,
            ChainConfig,
            AllowUnprotectedTxs,
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
                            "evmDenom" | "evm_denom" => Ok(GeneratedField::EvmDenom),
                            "enableCreate" | "enable_create" => Ok(GeneratedField::EnableCreate),
                            "enableCall" | "enable_call" => Ok(GeneratedField::EnableCall),
                            "extraEips" | "extra_eips" => Ok(GeneratedField::ExtraEips),
                            "chainConfig" | "chain_config" => Ok(GeneratedField::ChainConfig),
                            "allowUnprotectedTxs" | "allow_unprotected_txs" => Ok(GeneratedField::AllowUnprotectedTxs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Params;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut evm_denom__ = None;
                let mut enable_create__ = None;
                let mut enable_call__ = None;
                let mut extra_eips__ = None;
                let mut chain_config__ = None;
                let mut allow_unprotected_txs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EvmDenom => {
                            if evm_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("evmDenom"));
                            }
                            evm_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EnableCreate => {
                            if enable_create__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableCreate"));
                            }
                            enable_create__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EnableCall => {
                            if enable_call__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableCall"));
                            }
                            enable_call__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExtraEips => {
                            if extra_eips__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extraEips"));
                            }
                            extra_eips__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ChainConfig => {
                            if chain_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainConfig"));
                            }
                            chain_config__ = map_.next_value()?;
                        }
                        GeneratedField::AllowUnprotectedTxs => {
                            if allow_unprotected_txs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowUnprotectedTxs"));
                            }
                            allow_unprotected_txs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    evm_denom: evm_denom__.unwrap_or_default(),
                    enable_create: enable_create__.unwrap_or_default(),
                    enable_call: enable_call__.unwrap_or_default(),
                    extra_eips: extra_eips__.unwrap_or_default(),
                    chain_config: chain_config__,
                    allow_unprotected_txs: allow_unprotected_txs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAccountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryAccountRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryAccountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAccountRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.balance.is_empty() {
            len += 1;
        }
        if !self.code_hash.is_empty() {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryAccountResponse", len)?;
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        if !self.code_hash.is_empty() {
            struct_ser.serialize_field("codeHash", &self.code_hash)?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", ToString::to_string(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "balance",
            "code_hash",
            "codeHash",
            "nonce",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balance,
            CodeHash,
            Nonce,
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
                            "balance" => Ok(GeneratedField::Balance),
                            "codeHash" | "code_hash" => Ok(GeneratedField::CodeHash),
                            "nonce" => Ok(GeneratedField::Nonce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balance__ = None;
                let mut code_hash__ = None;
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeHash => {
                            if code_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeHash"));
                            }
                            code_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryAccountResponse {
                    balance: balance__.unwrap_or_default(),
                    code_hash: code_hash__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBalanceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryBalanceRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBalanceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBalanceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryBalanceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBalanceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBalanceRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryBalanceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBalanceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.balance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryBalanceResponse", len)?;
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBalanceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "balance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balance,
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
                            "balance" => Ok(GeneratedField::Balance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBalanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryBalanceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBalanceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBalanceResponse {
                    balance: balance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryBalanceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBaseFeeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryBaseFeeRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBaseFeeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBaseFeeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryBaseFeeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBaseFeeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBaseFeeRequest {
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryBaseFeeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBaseFeeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.base_fee.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryBaseFeeResponse", len)?;
        if !self.base_fee.is_empty() {
            struct_ser.serialize_field("baseFee", &self.base_fee)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBaseFeeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_fee",
            "baseFee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseFee,
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
                            "baseFee" | "base_fee" => Ok(GeneratedField::BaseFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBaseFeeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryBaseFeeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBaseFeeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseFee => {
                            if base_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseFee"));
                            }
                            base_fee__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBaseFeeResponse {
                    base_fee: base_fee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryBaseFeeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryCodeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryCodeRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCodeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCodeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryCodeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryCodeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryCodeRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryCodeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryCodeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryCodeResponse", len)?;
        if !self.code.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("code", pbjson::private::base64::encode(&self.code).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCodeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
                            "code" => Ok(GeneratedField::Code),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCodeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryCodeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryCodeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryCodeResponse {
                    code: code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryCodeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryCosmosAccountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryCosmosAccountRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCosmosAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCosmosAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryCosmosAccountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryCosmosAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryCosmosAccountRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryCosmosAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryCosmosAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cosmos_address.is_empty() {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        if self.account_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryCosmosAccountResponse", len)?;
        if !self.cosmos_address.is_empty() {
            struct_ser.serialize_field("cosmosAddress", &self.cosmos_address)?;
        }
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sequence", ToString::to_string(&self.sequence).as_str())?;
        }
        if self.account_number != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("accountNumber", ToString::to_string(&self.account_number).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCosmosAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cosmos_address",
            "cosmosAddress",
            "sequence",
            "account_number",
            "accountNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CosmosAddress,
            Sequence,
            AccountNumber,
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
                            "cosmosAddress" | "cosmos_address" => Ok(GeneratedField::CosmosAddress),
                            "sequence" => Ok(GeneratedField::Sequence),
                            "accountNumber" | "account_number" => Ok(GeneratedField::AccountNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCosmosAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryCosmosAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryCosmosAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cosmos_address__ = None;
                let mut sequence__ = None;
                let mut account_number__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CosmosAddress => {
                            if cosmos_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cosmosAddress"));
                            }
                            cosmos_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AccountNumber => {
                            if account_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountNumber"));
                            }
                            account_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryCosmosAccountResponse {
                    cosmos_address: cosmos_address__.unwrap_or_default(),
                    sequence: sequence__.unwrap_or_default(),
                    account_number: account_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryCosmosAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryParamsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryParamsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
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
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryParamsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse {
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryParamsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryStorageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryStorageRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryStorageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Key,
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
                            "address" => Ok(GeneratedField::Address),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryStorageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryStorageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryStorageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryStorageRequest {
                    address: address__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryStorageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryStorageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryStorageResponse", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryStorageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryStorageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryStorageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryStorageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryStorageResponse {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryStorageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryTraceBlockRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.txs.is_empty() {
            len += 1;
        }
        if self.trace_config.is_some() {
            len += 1;
        }
        if self.block_number != 0 {
            len += 1;
        }
        if !self.block_hash.is_empty() {
            len += 1;
        }
        if self.block_time.is_some() {
            len += 1;
        }
        if !self.proposer_address.is_empty() {
            len += 1;
        }
        if self.chain_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryTraceBlockRequest", len)?;
        if !self.txs.is_empty() {
            struct_ser.serialize_field("txs", &self.txs)?;
        }
        if let Some(v) = self.trace_config.as_ref() {
            struct_ser.serialize_field("traceConfig", v)?;
        }
        if self.block_number != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("blockNumber", ToString::to_string(&self.block_number).as_str())?;
        }
        if !self.block_hash.is_empty() {
            struct_ser.serialize_field("blockHash", &self.block_hash)?;
        }
        if let Some(v) = self.block_time.as_ref() {
            struct_ser.serialize_field("blockTime", v)?;
        }
        if !self.proposer_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("proposerAddress", pbjson::private::base64::encode(&self.proposer_address).as_str())?;
        }
        if self.chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryTraceBlockRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txs",
            "trace_config",
            "traceConfig",
            "block_number",
            "blockNumber",
            "block_hash",
            "blockHash",
            "block_time",
            "blockTime",
            "proposer_address",
            "proposerAddress",
            "chain_id",
            "chainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txs,
            TraceConfig,
            BlockNumber,
            BlockHash,
            BlockTime,
            ProposerAddress,
            ChainId,
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
                            "txs" => Ok(GeneratedField::Txs),
                            "traceConfig" | "trace_config" => Ok(GeneratedField::TraceConfig),
                            "blockNumber" | "block_number" => Ok(GeneratedField::BlockNumber),
                            "blockHash" | "block_hash" => Ok(GeneratedField::BlockHash),
                            "blockTime" | "block_time" => Ok(GeneratedField::BlockTime),
                            "proposerAddress" | "proposer_address" => Ok(GeneratedField::ProposerAddress),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryTraceBlockRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryTraceBlockRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryTraceBlockRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut txs__ = None;
                let mut trace_config__ = None;
                let mut block_number__ = None;
                let mut block_hash__ = None;
                let mut block_time__ = None;
                let mut proposer_address__ = None;
                let mut chain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Txs => {
                            if txs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txs"));
                            }
                            txs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TraceConfig => {
                            if trace_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceConfig"));
                            }
                            trace_config__ = map_.next_value()?;
                        }
                        GeneratedField::BlockNumber => {
                            if block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockNumber"));
                            }
                            block_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockHash => {
                            if block_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHash"));
                            }
                            block_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockTime => {
                            if block_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockTime"));
                            }
                            block_time__ = map_.next_value()?;
                        }
                        GeneratedField::ProposerAddress => {
                            if proposer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerAddress"));
                            }
                            proposer_address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryTraceBlockRequest {
                    txs: txs__.unwrap_or_default(),
                    trace_config: trace_config__,
                    block_number: block_number__.unwrap_or_default(),
                    block_hash: block_hash__.unwrap_or_default(),
                    block_time: block_time__,
                    proposer_address: proposer_address__.unwrap_or_default(),
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryTraceBlockRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryTraceBlockResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryTraceBlockResponse", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryTraceBlockResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryTraceBlockResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryTraceBlockResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryTraceBlockResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryTraceBlockResponse {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryTraceBlockResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryTraceTxRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.msg.is_some() {
            len += 1;
        }
        if self.trace_config.is_some() {
            len += 1;
        }
        if !self.predecessors.is_empty() {
            len += 1;
        }
        if self.block_number != 0 {
            len += 1;
        }
        if !self.block_hash.is_empty() {
            len += 1;
        }
        if self.block_time.is_some() {
            len += 1;
        }
        if !self.proposer_address.is_empty() {
            len += 1;
        }
        if self.chain_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryTraceTxRequest", len)?;
        if let Some(v) = self.msg.as_ref() {
            struct_ser.serialize_field("msg", v)?;
        }
        if let Some(v) = self.trace_config.as_ref() {
            struct_ser.serialize_field("traceConfig", v)?;
        }
        if !self.predecessors.is_empty() {
            struct_ser.serialize_field("predecessors", &self.predecessors)?;
        }
        if self.block_number != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("blockNumber", ToString::to_string(&self.block_number).as_str())?;
        }
        if !self.block_hash.is_empty() {
            struct_ser.serialize_field("blockHash", &self.block_hash)?;
        }
        if let Some(v) = self.block_time.as_ref() {
            struct_ser.serialize_field("blockTime", v)?;
        }
        if !self.proposer_address.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("proposerAddress", pbjson::private::base64::encode(&self.proposer_address).as_str())?;
        }
        if self.chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryTraceTxRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "msg",
            "trace_config",
            "traceConfig",
            "predecessors",
            "block_number",
            "blockNumber",
            "block_hash",
            "blockHash",
            "block_time",
            "blockTime",
            "proposer_address",
            "proposerAddress",
            "chain_id",
            "chainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Msg,
            TraceConfig,
            Predecessors,
            BlockNumber,
            BlockHash,
            BlockTime,
            ProposerAddress,
            ChainId,
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
                            "msg" => Ok(GeneratedField::Msg),
                            "traceConfig" | "trace_config" => Ok(GeneratedField::TraceConfig),
                            "predecessors" => Ok(GeneratedField::Predecessors),
                            "blockNumber" | "block_number" => Ok(GeneratedField::BlockNumber),
                            "blockHash" | "block_hash" => Ok(GeneratedField::BlockHash),
                            "blockTime" | "block_time" => Ok(GeneratedField::BlockTime),
                            "proposerAddress" | "proposer_address" => Ok(GeneratedField::ProposerAddress),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryTraceTxRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryTraceTxRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryTraceTxRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msg__ = None;
                let mut trace_config__ = None;
                let mut predecessors__ = None;
                let mut block_number__ = None;
                let mut block_hash__ = None;
                let mut block_time__ = None;
                let mut proposer_address__ = None;
                let mut chain_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Msg => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("msg"));
                            }
                            msg__ = map_.next_value()?;
                        }
                        GeneratedField::TraceConfig => {
                            if trace_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceConfig"));
                            }
                            trace_config__ = map_.next_value()?;
                        }
                        GeneratedField::Predecessors => {
                            if predecessors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predecessors"));
                            }
                            predecessors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockNumber => {
                            if block_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockNumber"));
                            }
                            block_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockHash => {
                            if block_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHash"));
                            }
                            block_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockTime => {
                            if block_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockTime"));
                            }
                            block_time__ = map_.next_value()?;
                        }
                        GeneratedField::ProposerAddress => {
                            if proposer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerAddress"));
                            }
                            proposer_address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryTraceTxRequest {
                    msg: msg__,
                    trace_config: trace_config__,
                    predecessors: predecessors__.unwrap_or_default(),
                    block_number: block_number__.unwrap_or_default(),
                    block_hash: block_hash__.unwrap_or_default(),
                    block_time: block_time__,
                    proposer_address: proposer_address__.unwrap_or_default(),
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryTraceTxRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryTraceTxResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryTraceTxResponse", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryTraceTxResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryTraceTxResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryTraceTxResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryTraceTxResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryTraceTxResponse {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryTraceTxResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryTxLogsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryTxLogsRequest", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryTxLogsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Pagination,
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
                            "hash" => Ok(GeneratedField::Hash),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryTxLogsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryTxLogsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryTxLogsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryTxLogsRequest {
                    hash: hash__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryTxLogsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryTxLogsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.logs.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryTxLogsResponse", len)?;
        if !self.logs.is_empty() {
            struct_ser.serialize_field("logs", &self.logs)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryTxLogsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "logs",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Logs,
            Pagination,
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
                            "logs" => Ok(GeneratedField::Logs),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryTxLogsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryTxLogsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryTxLogsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut logs__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Logs => {
                            if logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logs"));
                            }
                            logs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryTxLogsResponse {
                    logs: logs__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryTxLogsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryValidatorAccountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cons_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryValidatorAccountRequest", len)?;
        if !self.cons_address.is_empty() {
            struct_ser.serialize_field("consAddress", &self.cons_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryValidatorAccountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cons_address",
            "consAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConsAddress,
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
                            "consAddress" | "cons_address" => Ok(GeneratedField::ConsAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryValidatorAccountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryValidatorAccountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryValidatorAccountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cons_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConsAddress => {
                            if cons_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consAddress"));
                            }
                            cons_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryValidatorAccountRequest {
                    cons_address: cons_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryValidatorAccountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryValidatorAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account_address.is_empty() {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        if self.account_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.QueryValidatorAccountResponse", len)?;
        if !self.account_address.is_empty() {
            struct_ser.serialize_field("accountAddress", &self.account_address)?;
        }
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sequence", ToString::to_string(&self.sequence).as_str())?;
        }
        if self.account_number != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("accountNumber", ToString::to_string(&self.account_number).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryValidatorAccountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account_address",
            "accountAddress",
            "sequence",
            "account_number",
            "accountNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccountAddress,
            Sequence,
            AccountNumber,
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
                            "accountAddress" | "account_address" => Ok(GeneratedField::AccountAddress),
                            "sequence" => Ok(GeneratedField::Sequence),
                            "accountNumber" | "account_number" => Ok(GeneratedField::AccountNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryValidatorAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.QueryValidatorAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryValidatorAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account_address__ = None;
                let mut sequence__ = None;
                let mut account_number__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccountAddress => {
                            if account_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountAddress"));
                            }
                            account_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AccountNumber => {
                            if account_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountNumber"));
                            }
                            account_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryValidatorAccountResponse {
                    account_address: account_address__.unwrap_or_default(),
                    sequence: sequence__.unwrap_or_default(),
                    account_number: account_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.QueryValidatorAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.State", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = State;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.State")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<State, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(State {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.State", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TraceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tracer.is_empty() {
            len += 1;
        }
        if !self.timeout.is_empty() {
            len += 1;
        }
        if self.reexec != 0 {
            len += 1;
        }
        if self.disable_stack {
            len += 1;
        }
        if self.disable_storage {
            len += 1;
        }
        if self.debug {
            len += 1;
        }
        if self.limit != 0 {
            len += 1;
        }
        if self.overrides.is_some() {
            len += 1;
        }
        if self.enable_memory {
            len += 1;
        }
        if self.enable_return_data {
            len += 1;
        }
        if !self.tracer_json_config.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.TraceConfig", len)?;
        if !self.tracer.is_empty() {
            struct_ser.serialize_field("tracer", &self.tracer)?;
        }
        if !self.timeout.is_empty() {
            struct_ser.serialize_field("timeout", &self.timeout)?;
        }
        if self.reexec != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("reexec", ToString::to_string(&self.reexec).as_str())?;
        }
        if self.disable_stack {
            struct_ser.serialize_field("disableStack", &self.disable_stack)?;
        }
        if self.disable_storage {
            struct_ser.serialize_field("disableStorage", &self.disable_storage)?;
        }
        if self.debug {
            struct_ser.serialize_field("debug", &self.debug)?;
        }
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if let Some(v) = self.overrides.as_ref() {
            struct_ser.serialize_field("overrides", v)?;
        }
        if self.enable_memory {
            struct_ser.serialize_field("enableMemory", &self.enable_memory)?;
        }
        if self.enable_return_data {
            struct_ser.serialize_field("enableReturnData", &self.enable_return_data)?;
        }
        if !self.tracer_json_config.is_empty() {
            struct_ser.serialize_field("tracerJsonConfig", &self.tracer_json_config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TraceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tracer",
            "timeout",
            "reexec",
            "disable_stack",
            "disableStack",
            "disable_storage",
            "disableStorage",
            "debug",
            "limit",
            "overrides",
            "enable_memory",
            "enableMemory",
            "enable_return_data",
            "enableReturnData",
            "tracer_json_config",
            "tracerJsonConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tracer,
            Timeout,
            Reexec,
            DisableStack,
            DisableStorage,
            Debug,
            Limit,
            Overrides,
            EnableMemory,
            EnableReturnData,
            TracerJsonConfig,
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
                            "tracer" => Ok(GeneratedField::Tracer),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "reexec" => Ok(GeneratedField::Reexec),
                            "disableStack" | "disable_stack" => Ok(GeneratedField::DisableStack),
                            "disableStorage" | "disable_storage" => Ok(GeneratedField::DisableStorage),
                            "debug" => Ok(GeneratedField::Debug),
                            "limit" => Ok(GeneratedField::Limit),
                            "overrides" => Ok(GeneratedField::Overrides),
                            "enableMemory" | "enable_memory" => Ok(GeneratedField::EnableMemory),
                            "enableReturnData" | "enable_return_data" => Ok(GeneratedField::EnableReturnData),
                            "tracerJsonConfig" | "tracer_json_config" => Ok(GeneratedField::TracerJsonConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TraceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.TraceConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TraceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tracer__ = None;
                let mut timeout__ = None;
                let mut reexec__ = None;
                let mut disable_stack__ = None;
                let mut disable_storage__ = None;
                let mut debug__ = None;
                let mut limit__ = None;
                let mut overrides__ = None;
                let mut enable_memory__ = None;
                let mut enable_return_data__ = None;
                let mut tracer_json_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tracer => {
                            if tracer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracer"));
                            }
                            tracer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reexec => {
                            if reexec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reexec"));
                            }
                            reexec__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DisableStack => {
                            if disable_stack__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableStack"));
                            }
                            disable_stack__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableStorage => {
                            if disable_storage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableStorage"));
                            }
                            disable_storage__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Debug => {
                            if debug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debug"));
                            }
                            debug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Overrides => {
                            if overrides__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            overrides__ = map_.next_value()?;
                        }
                        GeneratedField::EnableMemory => {
                            if enable_memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableMemory"));
                            }
                            enable_memory__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EnableReturnData => {
                            if enable_return_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableReturnData"));
                            }
                            enable_return_data__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TracerJsonConfig => {
                            if tracer_json_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracerJsonConfig"));
                            }
                            tracer_json_config__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TraceConfig {
                    tracer: tracer__.unwrap_or_default(),
                    timeout: timeout__.unwrap_or_default(),
                    reexec: reexec__.unwrap_or_default(),
                    disable_stack: disable_stack__.unwrap_or_default(),
                    disable_storage: disable_storage__.unwrap_or_default(),
                    debug: debug__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    overrides: overrides__,
                    enable_memory: enable_memory__.unwrap_or_default(),
                    enable_return_data: enable_return_data__.unwrap_or_default(),
                    tracer_json_config: tracer_json_config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.TraceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransactionLogs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if !self.logs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.TransactionLogs", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if !self.logs.is_empty() {
            struct_ser.serialize_field("logs", &self.logs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransactionLogs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "logs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Logs,
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
                            "hash" => Ok(GeneratedField::Hash),
                            "logs" => Ok(GeneratedField::Logs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransactionLogs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct ethermint.evm.v1.TransactionLogs")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TransactionLogs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut logs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Logs => {
                            if logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logs"));
                            }
                            logs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TransactionLogs {
                    hash: hash__.unwrap_or_default(),
                    logs: logs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.TransactionLogs", FIELDS, GeneratedVisitor)
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
        if !self.contract_address.is_empty() {
            len += 1;
        }
        if !self.bloom.is_empty() {
            len += 1;
        }
        if self.tx_logs.is_some() {
            len += 1;
        }
        if !self.ret.is_empty() {
            len += 1;
        }
        if self.reverted {
            len += 1;
        }
        if self.gas_used != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("ethermint.evm.v1.TxResult", len)?;
        if !self.contract_address.is_empty() {
            struct_ser.serialize_field("contractAddress", &self.contract_address)?;
        }
        if !self.bloom.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("bloom", pbjson::private::base64::encode(&self.bloom).as_str())?;
        }
        if let Some(v) = self.tx_logs.as_ref() {
            struct_ser.serialize_field("txLogs", v)?;
        }
        if !self.ret.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("ret", pbjson::private::base64::encode(&self.ret).as_str())?;
        }
        if self.reverted {
            struct_ser.serialize_field("reverted", &self.reverted)?;
        }
        if self.gas_used != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("gasUsed", ToString::to_string(&self.gas_used).as_str())?;
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
            "contract_address",
            "contractAddress",
            "bloom",
            "tx_logs",
            "txLogs",
            "ret",
            "reverted",
            "gas_used",
            "gasUsed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContractAddress,
            Bloom,
            TxLogs,
            Ret,
            Reverted,
            GasUsed,
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
                            "contractAddress" | "contract_address" => Ok(GeneratedField::ContractAddress),
                            "bloom" => Ok(GeneratedField::Bloom),
                            "txLogs" | "tx_logs" => Ok(GeneratedField::TxLogs),
                            "ret" => Ok(GeneratedField::Ret),
                            "reverted" => Ok(GeneratedField::Reverted),
                            "gasUsed" | "gas_used" => Ok(GeneratedField::GasUsed),
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
                formatter.write_str("struct ethermint.evm.v1.TxResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut contract_address__ = None;
                let mut bloom__ = None;
                let mut tx_logs__ = None;
                let mut ret__ = None;
                let mut reverted__ = None;
                let mut gas_used__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContractAddress => {
                            if contract_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contractAddress"));
                            }
                            contract_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bloom => {
                            if bloom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bloom"));
                            }
                            bloom__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TxLogs => {
                            if tx_logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txLogs"));
                            }
                            tx_logs__ = map_.next_value()?;
                        }
                        GeneratedField::Ret => {
                            if ret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ret"));
                            }
                            ret__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Reverted => {
                            if reverted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reverted"));
                            }
                            reverted__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GasUsed => {
                            if gas_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasUsed"));
                            }
                            gas_used__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TxResult {
                    contract_address: contract_address__.unwrap_or_default(),
                    bloom: bloom__.unwrap_or_default(),
                    tx_logs: tx_logs__,
                    ret: ret__.unwrap_or_default(),
                    reverted: reverted__.unwrap_or_default(),
                    gas_used: gas_used__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("ethermint.evm.v1.TxResult", FIELDS, GeneratedVisitor)
    }
}
