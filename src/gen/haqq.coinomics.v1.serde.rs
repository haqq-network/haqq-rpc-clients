// @generated
impl serde::Serialize for GenesisState {
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
        if !self.inflation.is_empty() {
            len += 1;
        }
        if self.era != 0 {
            len += 1;
        }
        if self.era_started_at_block != 0 {
            len += 1;
        }
        if self.era_target_mint.is_some() {
            len += 1;
        }
        if self.era_closing_supply.is_some() {
            len += 1;
        }
        if self.max_supply.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.coinomics.v1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.inflation.is_empty() {
            struct_ser.serialize_field("inflation", &self.inflation)?;
        }
        if self.era != 0 {
            struct_ser.serialize_field("era", ToString::to_string(&self.era).as_str())?;
        }
        if self.era_started_at_block != 0 {
            struct_ser.serialize_field("eraStartedAtBlock", ToString::to_string(&self.era_started_at_block).as_str())?;
        }
        if let Some(v) = self.era_target_mint.as_ref() {
            struct_ser.serialize_field("eraTargetMint", v)?;
        }
        if let Some(v) = self.era_closing_supply.as_ref() {
            struct_ser.serialize_field("eraClosingSupply", v)?;
        }
        if let Some(v) = self.max_supply.as_ref() {
            struct_ser.serialize_field("maxSupply", v)?;
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
            "params",
            "inflation",
            "era",
            "era_started_at_block",
            "eraStartedAtBlock",
            "era_target_mint",
            "eraTargetMint",
            "era_closing_supply",
            "eraClosingSupply",
            "max_supply",
            "maxSupply",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            Inflation,
            Era,
            EraStartedAtBlock,
            EraTargetMint,
            EraClosingSupply,
            MaxSupply,
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
                            "inflation" => Ok(GeneratedField::Inflation),
                            "era" => Ok(GeneratedField::Era),
                            "eraStartedAtBlock" | "era_started_at_block" => Ok(GeneratedField::EraStartedAtBlock),
                            "eraTargetMint" | "era_target_mint" => Ok(GeneratedField::EraTargetMint),
                            "eraClosingSupply" | "era_closing_supply" => Ok(GeneratedField::EraClosingSupply),
                            "maxSupply" | "max_supply" => Ok(GeneratedField::MaxSupply),
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
                formatter.write_str("struct haqq.coinomics.v1.GenesisState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GenesisState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut inflation__ = None;
                let mut era__ = None;
                let mut era_started_at_block__ = None;
                let mut era_target_mint__ = None;
                let mut era_closing_supply__ = None;
                let mut max_supply__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::Inflation => {
                            if inflation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inflation"));
                            }
                            inflation__ = Some(map.next_value()?);
                        }
                        GeneratedField::Era => {
                            if era__.is_some() {
                                return Err(serde::de::Error::duplicate_field("era"));
                            }
                            era__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EraStartedAtBlock => {
                            if era_started_at_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eraStartedAtBlock"));
                            }
                            era_started_at_block__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EraTargetMint => {
                            if era_target_mint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eraTargetMint"));
                            }
                            era_target_mint__ = map.next_value()?;
                        }
                        GeneratedField::EraClosingSupply => {
                            if era_closing_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eraClosingSupply"));
                            }
                            era_closing_supply__ = map.next_value()?;
                        }
                        GeneratedField::MaxSupply => {
                            if max_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSupply"));
                            }
                            max_supply__ = map.next_value()?;
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    inflation: inflation__.unwrap_or_default(),
                    era: era__.unwrap_or_default(),
                    era_started_at_block: era_started_at_block__.unwrap_or_default(),
                    era_target_mint: era_target_mint__,
                    era_closing_supply: era_closing_supply__,
                    max_supply: max_supply__,
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.GenesisState", FIELDS, GeneratedVisitor)
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
        if !self.mint_denom.is_empty() {
            len += 1;
        }
        if self.blocks_per_era != 0 {
            len += 1;
        }
        if self.enable_coinomics {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.coinomics.v1.Params", len)?;
        if !self.mint_denom.is_empty() {
            struct_ser.serialize_field("mintDenom", &self.mint_denom)?;
        }
        if self.blocks_per_era != 0 {
            struct_ser.serialize_field("blocksPerEra", ToString::to_string(&self.blocks_per_era).as_str())?;
        }
        if self.enable_coinomics {
            struct_ser.serialize_field("enableCoinomics", &self.enable_coinomics)?;
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
            "mint_denom",
            "mintDenom",
            "blocks_per_era",
            "blocksPerEra",
            "enable_coinomics",
            "enableCoinomics",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MintDenom,
            BlocksPerEra,
            EnableCoinomics,
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
                            "mintDenom" | "mint_denom" => Ok(GeneratedField::MintDenom),
                            "blocksPerEra" | "blocks_per_era" => Ok(GeneratedField::BlocksPerEra),
                            "enableCoinomics" | "enable_coinomics" => Ok(GeneratedField::EnableCoinomics),
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
                formatter.write_str("struct haqq.coinomics.v1.Params")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mint_denom__ = None;
                let mut blocks_per_era__ = None;
                let mut enable_coinomics__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MintDenom => {
                            if mint_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintDenom"));
                            }
                            mint_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::BlocksPerEra => {
                            if blocks_per_era__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blocksPerEra"));
                            }
                            blocks_per_era__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EnableCoinomics => {
                            if enable_coinomics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableCoinomics"));
                            }
                            enable_coinomics__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    mint_denom: mint_denom__.unwrap_or_default(),
                    blocks_per_era: blocks_per_era__.unwrap_or_default(),
                    enable_coinomics: enable_coinomics__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryEraClosingSupplyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("haqq.coinomics.v1.QueryEraClosingSupplyRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEraClosingSupplyRequest {
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
            type Value = QueryEraClosingSupplyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.coinomics.v1.QueryEraClosingSupplyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryEraClosingSupplyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryEraClosingSupplyRequest {
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.QueryEraClosingSupplyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryEraClosingSupplyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.era_closing_supply.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.coinomics.v1.QueryEraClosingSupplyResponse", len)?;
        if let Some(v) = self.era_closing_supply.as_ref() {
            struct_ser.serialize_field("eraClosingSupply", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEraClosingSupplyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "era_closing_supply",
            "eraClosingSupply",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EraClosingSupply,
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
                            "eraClosingSupply" | "era_closing_supply" => Ok(GeneratedField::EraClosingSupply),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEraClosingSupplyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.coinomics.v1.QueryEraClosingSupplyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryEraClosingSupplyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut era_closing_supply__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EraClosingSupply => {
                            if era_closing_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eraClosingSupply"));
                            }
                            era_closing_supply__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryEraClosingSupplyResponse {
                    era_closing_supply: era_closing_supply__,
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.QueryEraClosingSupplyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryEraRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("haqq.coinomics.v1.QueryEraRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEraRequest {
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
            type Value = QueryEraRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.coinomics.v1.QueryEraRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryEraRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryEraRequest {
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.QueryEraRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryEraResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.era != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.coinomics.v1.QueryEraResponse", len)?;
        if self.era != 0 {
            struct_ser.serialize_field("era", ToString::to_string(&self.era).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryEraResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "era",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Era,
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
                            "era" => Ok(GeneratedField::Era),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEraResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.coinomics.v1.QueryEraResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryEraResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut era__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Era => {
                            if era__.is_some() {
                                return Err(serde::de::Error::duplicate_field("era"));
                            }
                            era__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryEraResponse {
                    era: era__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.QueryEraResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryInflationRateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("haqq.coinomics.v1.QueryInflationRateRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryInflationRateRequest {
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
            type Value = QueryInflationRateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.coinomics.v1.QueryInflationRateRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryInflationRateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryInflationRateRequest {
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.QueryInflationRateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryInflationRateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.inflation_rate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.coinomics.v1.QueryInflationRateResponse", len)?;
        if !self.inflation_rate.is_empty() {
            struct_ser.serialize_field("inflationRate", &self.inflation_rate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryInflationRateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inflation_rate",
            "inflationRate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InflationRate,
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
                            "inflationRate" | "inflation_rate" => Ok(GeneratedField::InflationRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryInflationRateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.coinomics.v1.QueryInflationRateResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryInflationRateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inflation_rate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InflationRate => {
                            if inflation_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inflationRate"));
                            }
                            inflation_rate__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryInflationRateResponse {
                    inflation_rate: inflation_rate__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.QueryInflationRateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryMaxSupplyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("haqq.coinomics.v1.QueryMaxSupplyRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryMaxSupplyRequest {
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
            type Value = QueryMaxSupplyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.coinomics.v1.QueryMaxSupplyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryMaxSupplyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryMaxSupplyRequest {
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.QueryMaxSupplyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryMaxSupplyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_supply.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.coinomics.v1.QueryMaxSupplyResponse", len)?;
        if let Some(v) = self.max_supply.as_ref() {
            struct_ser.serialize_field("maxSupply", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryMaxSupplyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_supply",
            "maxSupply",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxSupply,
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
                            "maxSupply" | "max_supply" => Ok(GeneratedField::MaxSupply),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryMaxSupplyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.coinomics.v1.QueryMaxSupplyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryMaxSupplyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_supply__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxSupply => {
                            if max_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSupply"));
                            }
                            max_supply__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryMaxSupplyResponse {
                    max_supply: max_supply__,
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.QueryMaxSupplyResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("haqq.coinomics.v1.QueryParamsRequest", len)?;
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
                formatter.write_str("struct haqq.coinomics.v1.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParamsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.QueryParamsRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("haqq.coinomics.v1.QueryParamsResponse", len)?;
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
                formatter.write_str("struct haqq.coinomics.v1.QueryParamsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse {
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("haqq.coinomics.v1.QueryParamsResponse", FIELDS, GeneratedVisitor)
    }
}
