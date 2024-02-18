// @generated
impl serde::Serialize for ClawbackVestingAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_vesting_account.is_some() {
            len += 1;
        }
        if !self.funder_address.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if !self.lockup_periods.is_empty() {
            len += 1;
        }
        if !self.vesting_periods.is_empty() {
            len += 1;
        }
        if !self.code_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.ClawbackVestingAccount", len)?;
        if let Some(v) = self.base_vesting_account.as_ref() {
            struct_ser.serialize_field("baseVestingAccount", v)?;
        }
        if !self.funder_address.is_empty() {
            struct_ser.serialize_field("funderAddress", &self.funder_address)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if !self.lockup_periods.is_empty() {
            struct_ser.serialize_field("lockupPeriods", &self.lockup_periods)?;
        }
        if !self.vesting_periods.is_empty() {
            struct_ser.serialize_field("vestingPeriods", &self.vesting_periods)?;
        }
        if !self.code_hash.is_empty() {
            struct_ser.serialize_field("codeHash", &self.code_hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClawbackVestingAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_vesting_account",
            "baseVestingAccount",
            "funder_address",
            "funderAddress",
            "start_time",
            "startTime",
            "lockup_periods",
            "lockupPeriods",
            "vesting_periods",
            "vestingPeriods",
            "code_hash",
            "codeHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseVestingAccount,
            FunderAddress,
            StartTime,
            LockupPeriods,
            VestingPeriods,
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
                            "baseVestingAccount" | "base_vesting_account" => Ok(GeneratedField::BaseVestingAccount),
                            "funderAddress" | "funder_address" => Ok(GeneratedField::FunderAddress),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "lockupPeriods" | "lockup_periods" => Ok(GeneratedField::LockupPeriods),
                            "vestingPeriods" | "vesting_periods" => Ok(GeneratedField::VestingPeriods),
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
            type Value = ClawbackVestingAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.ClawbackVestingAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClawbackVestingAccount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_vesting_account__ = None;
                let mut funder_address__ = None;
                let mut start_time__ = None;
                let mut lockup_periods__ = None;
                let mut vesting_periods__ = None;
                let mut code_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseVestingAccount => {
                            if base_vesting_account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseVestingAccount"));
                            }
                            base_vesting_account__ = map_.next_value()?;
                        }
                        GeneratedField::FunderAddress => {
                            if funder_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funderAddress"));
                            }
                            funder_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::LockupPeriods => {
                            if lockup_periods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lockupPeriods"));
                            }
                            lockup_periods__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VestingPeriods => {
                            if vesting_periods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vestingPeriods"));
                            }
                            vesting_periods__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeHash => {
                            if code_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeHash"));
                            }
                            code_hash__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClawbackVestingAccount {
                    base_vesting_account: base_vesting_account__,
                    funder_address: funder_address__.unwrap_or_default(),
                    start_time: start_time__,
                    lockup_periods: lockup_periods__.unwrap_or_default(),
                    vesting_periods: vesting_periods__.unwrap_or_default(),
                    code_hash: code_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.ClawbackVestingAccount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventClawback {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.funder.is_empty() {
            len += 1;
        }
        if !self.account.is_empty() {
            len += 1;
        }
        if !self.destination.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.EventClawback", len)?;
        if !self.funder.is_empty() {
            struct_ser.serialize_field("funder", &self.funder)?;
        }
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        if !self.destination.is_empty() {
            struct_ser.serialize_field("destination", &self.destination)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventClawback {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "funder",
            "account",
            "destination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Funder,
            Account,
            Destination,
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
                            "funder" => Ok(GeneratedField::Funder),
                            "account" => Ok(GeneratedField::Account),
                            "destination" => Ok(GeneratedField::Destination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventClawback;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.EventClawback")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventClawback, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut funder__ = None;
                let mut account__ = None;
                let mut destination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Funder => {
                            if funder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funder"));
                            }
                            funder__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Destination => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            destination__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventClawback {
                    funder: funder__.unwrap_or_default(),
                    account: account__.unwrap_or_default(),
                    destination: destination__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.EventClawback", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventCreateClawbackVestingAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.coins.is_empty() {
            len += 1;
        }
        if !self.start_time.is_empty() {
            len += 1;
        }
        if !self.merge.is_empty() {
            len += 1;
        }
        if !self.account.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.EventCreateClawbackVestingAccount", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.coins.is_empty() {
            struct_ser.serialize_field("coins", &self.coins)?;
        }
        if !self.start_time.is_empty() {
            struct_ser.serialize_field("startTime", &self.start_time)?;
        }
        if !self.merge.is_empty() {
            struct_ser.serialize_field("merge", &self.merge)?;
        }
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventCreateClawbackVestingAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "coins",
            "start_time",
            "startTime",
            "merge",
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Coins,
            StartTime,
            Merge,
            Account,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "coins" => Ok(GeneratedField::Coins),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "merge" => Ok(GeneratedField::Merge),
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventCreateClawbackVestingAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.EventCreateClawbackVestingAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventCreateClawbackVestingAccount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut coins__ = None;
                let mut start_time__ = None;
                let mut merge__ = None;
                let mut account__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Coins => {
                            if coins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coins"));
                            }
                            coins__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Merge => {
                            if merge__.is_some() {
                                return Err(serde::de::Error::duplicate_field("merge"));
                            }
                            merge__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventCreateClawbackVestingAccount {
                    sender: sender__.unwrap_or_default(),
                    coins: coins__.unwrap_or_default(),
                    start_time: start_time__.unwrap_or_default(),
                    merge: merge__.unwrap_or_default(),
                    account: account__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.EventCreateClawbackVestingAccount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateVestingFunder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.funder.is_empty() {
            len += 1;
        }
        if !self.account.is_empty() {
            len += 1;
        }
        if !self.new_funder.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.EventUpdateVestingFunder", len)?;
        if !self.funder.is_empty() {
            struct_ser.serialize_field("funder", &self.funder)?;
        }
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
        }
        if !self.new_funder.is_empty() {
            struct_ser.serialize_field("newFunder", &self.new_funder)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateVestingFunder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "funder",
            "account",
            "new_funder",
            "newFunder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Funder,
            Account,
            NewFunder,
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
                            "funder" => Ok(GeneratedField::Funder),
                            "account" => Ok(GeneratedField::Account),
                            "newFunder" | "new_funder" => Ok(GeneratedField::NewFunder),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventUpdateVestingFunder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.EventUpdateVestingFunder")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventUpdateVestingFunder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut funder__ = None;
                let mut account__ = None;
                let mut new_funder__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Funder => {
                            if funder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funder"));
                            }
                            funder__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewFunder => {
                            if new_funder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newFunder"));
                            }
                            new_funder__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventUpdateVestingFunder {
                    funder: funder__.unwrap_or_default(),
                    account: account__.unwrap_or_default(),
                    new_funder: new_funder__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.EventUpdateVestingFunder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgClawback {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.funder_address.is_empty() {
            len += 1;
        }
        if !self.account_address.is_empty() {
            len += 1;
        }
        if !self.dest_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.MsgClawback", len)?;
        if !self.funder_address.is_empty() {
            struct_ser.serialize_field("funderAddress", &self.funder_address)?;
        }
        if !self.account_address.is_empty() {
            struct_ser.serialize_field("accountAddress", &self.account_address)?;
        }
        if !self.dest_address.is_empty() {
            struct_ser.serialize_field("destAddress", &self.dest_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgClawback {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "funder_address",
            "funderAddress",
            "account_address",
            "accountAddress",
            "dest_address",
            "destAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FunderAddress,
            AccountAddress,
            DestAddress,
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
                            "funderAddress" | "funder_address" => Ok(GeneratedField::FunderAddress),
                            "accountAddress" | "account_address" => Ok(GeneratedField::AccountAddress),
                            "destAddress" | "dest_address" => Ok(GeneratedField::DestAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgClawback;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.MsgClawback")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgClawback, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut funder_address__ = None;
                let mut account_address__ = None;
                let mut dest_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FunderAddress => {
                            if funder_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funderAddress"));
                            }
                            funder_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccountAddress => {
                            if account_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountAddress"));
                            }
                            account_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestAddress => {
                            if dest_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destAddress"));
                            }
                            dest_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgClawback {
                    funder_address: funder_address__.unwrap_or_default(),
                    account_address: account_address__.unwrap_or_default(),
                    dest_address: dest_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.MsgClawback", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgClawbackResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("haqq.vesting.v1.MsgClawbackResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgClawbackResponse {
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
            type Value = MsgClawbackResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.MsgClawbackResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgClawbackResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgClawbackResponse {
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.MsgClawbackResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgConvertIntoVestingAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from_address.is_empty() {
            len += 1;
        }
        if !self.to_address.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if !self.lockup_periods.is_empty() {
            len += 1;
        }
        if !self.vesting_periods.is_empty() {
            len += 1;
        }
        if self.merge {
            len += 1;
        }
        if self.stake {
            len += 1;
        }
        if !self.validator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.MsgConvertIntoVestingAccount", len)?;
        if !self.from_address.is_empty() {
            struct_ser.serialize_field("fromAddress", &self.from_address)?;
        }
        if !self.to_address.is_empty() {
            struct_ser.serialize_field("toAddress", &self.to_address)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if !self.lockup_periods.is_empty() {
            struct_ser.serialize_field("lockupPeriods", &self.lockup_periods)?;
        }
        if !self.vesting_periods.is_empty() {
            struct_ser.serialize_field("vestingPeriods", &self.vesting_periods)?;
        }
        if self.merge {
            struct_ser.serialize_field("merge", &self.merge)?;
        }
        if self.stake {
            struct_ser.serialize_field("stake", &self.stake)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgConvertIntoVestingAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from_address",
            "fromAddress",
            "to_address",
            "toAddress",
            "start_time",
            "startTime",
            "lockup_periods",
            "lockupPeriods",
            "vesting_periods",
            "vestingPeriods",
            "merge",
            "stake",
            "validator_address",
            "validatorAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FromAddress,
            ToAddress,
            StartTime,
            LockupPeriods,
            VestingPeriods,
            Merge,
            Stake,
            ValidatorAddress,
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
                            "fromAddress" | "from_address" => Ok(GeneratedField::FromAddress),
                            "toAddress" | "to_address" => Ok(GeneratedField::ToAddress),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "lockupPeriods" | "lockup_periods" => Ok(GeneratedField::LockupPeriods),
                            "vestingPeriods" | "vesting_periods" => Ok(GeneratedField::VestingPeriods),
                            "merge" => Ok(GeneratedField::Merge),
                            "stake" => Ok(GeneratedField::Stake),
                            "validatorAddress" | "validator_address" => Ok(GeneratedField::ValidatorAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgConvertIntoVestingAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.MsgConvertIntoVestingAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgConvertIntoVestingAccount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from_address__ = None;
                let mut to_address__ = None;
                let mut start_time__ = None;
                let mut lockup_periods__ = None;
                let mut vesting_periods__ = None;
                let mut merge__ = None;
                let mut stake__ = None;
                let mut validator_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FromAddress => {
                            if from_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromAddress"));
                            }
                            from_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ToAddress => {
                            if to_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toAddress"));
                            }
                            to_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::LockupPeriods => {
                            if lockup_periods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lockupPeriods"));
                            }
                            lockup_periods__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VestingPeriods => {
                            if vesting_periods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vestingPeriods"));
                            }
                            vesting_periods__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Merge => {
                            if merge__.is_some() {
                                return Err(serde::de::Error::duplicate_field("merge"));
                            }
                            merge__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Stake => {
                            if stake__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stake"));
                            }
                            stake__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorAddress"));
                            }
                            validator_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgConvertIntoVestingAccount {
                    from_address: from_address__.unwrap_or_default(),
                    to_address: to_address__.unwrap_or_default(),
                    start_time: start_time__,
                    lockup_periods: lockup_periods__.unwrap_or_default(),
                    vesting_periods: vesting_periods__.unwrap_or_default(),
                    merge: merge__.unwrap_or_default(),
                    stake: stake__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.MsgConvertIntoVestingAccount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgConvertIntoVestingAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("haqq.vesting.v1.MsgConvertIntoVestingAccountResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgConvertIntoVestingAccountResponse {
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
            type Value = MsgConvertIntoVestingAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.MsgConvertIntoVestingAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgConvertIntoVestingAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgConvertIntoVestingAccountResponse {
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.MsgConvertIntoVestingAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgConvertVestingAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vesting_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.MsgConvertVestingAccount", len)?;
        if !self.vesting_address.is_empty() {
            struct_ser.serialize_field("vestingAddress", &self.vesting_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgConvertVestingAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vesting_address",
            "vestingAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VestingAddress,
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
                            "vestingAddress" | "vesting_address" => Ok(GeneratedField::VestingAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgConvertVestingAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.MsgConvertVestingAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgConvertVestingAccount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vesting_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VestingAddress => {
                            if vesting_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vestingAddress"));
                            }
                            vesting_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgConvertVestingAccount {
                    vesting_address: vesting_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.MsgConvertVestingAccount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgConvertVestingAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("haqq.vesting.v1.MsgConvertVestingAccountResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgConvertVestingAccountResponse {
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
            type Value = MsgConvertVestingAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.MsgConvertVestingAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgConvertVestingAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgConvertVestingAccountResponse {
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.MsgConvertVestingAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateClawbackVestingAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from_address.is_empty() {
            len += 1;
        }
        if !self.to_address.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if !self.lockup_periods.is_empty() {
            len += 1;
        }
        if !self.vesting_periods.is_empty() {
            len += 1;
        }
        if self.merge {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.MsgCreateClawbackVestingAccount", len)?;
        if !self.from_address.is_empty() {
            struct_ser.serialize_field("fromAddress", &self.from_address)?;
        }
        if !self.to_address.is_empty() {
            struct_ser.serialize_field("toAddress", &self.to_address)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if !self.lockup_periods.is_empty() {
            struct_ser.serialize_field("lockupPeriods", &self.lockup_periods)?;
        }
        if !self.vesting_periods.is_empty() {
            struct_ser.serialize_field("vestingPeriods", &self.vesting_periods)?;
        }
        if self.merge {
            struct_ser.serialize_field("merge", &self.merge)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateClawbackVestingAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from_address",
            "fromAddress",
            "to_address",
            "toAddress",
            "start_time",
            "startTime",
            "lockup_periods",
            "lockupPeriods",
            "vesting_periods",
            "vestingPeriods",
            "merge",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FromAddress,
            ToAddress,
            StartTime,
            LockupPeriods,
            VestingPeriods,
            Merge,
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
                            "fromAddress" | "from_address" => Ok(GeneratedField::FromAddress),
                            "toAddress" | "to_address" => Ok(GeneratedField::ToAddress),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "lockupPeriods" | "lockup_periods" => Ok(GeneratedField::LockupPeriods),
                            "vestingPeriods" | "vesting_periods" => Ok(GeneratedField::VestingPeriods),
                            "merge" => Ok(GeneratedField::Merge),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateClawbackVestingAccount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.MsgCreateClawbackVestingAccount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCreateClawbackVestingAccount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from_address__ = None;
                let mut to_address__ = None;
                let mut start_time__ = None;
                let mut lockup_periods__ = None;
                let mut vesting_periods__ = None;
                let mut merge__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FromAddress => {
                            if from_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromAddress"));
                            }
                            from_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ToAddress => {
                            if to_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toAddress"));
                            }
                            to_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::LockupPeriods => {
                            if lockup_periods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lockupPeriods"));
                            }
                            lockup_periods__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VestingPeriods => {
                            if vesting_periods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vestingPeriods"));
                            }
                            vesting_periods__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Merge => {
                            if merge__.is_some() {
                                return Err(serde::de::Error::duplicate_field("merge"));
                            }
                            merge__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateClawbackVestingAccount {
                    from_address: from_address__.unwrap_or_default(),
                    to_address: to_address__.unwrap_or_default(),
                    start_time: start_time__,
                    lockup_periods: lockup_periods__.unwrap_or_default(),
                    vesting_periods: vesting_periods__.unwrap_or_default(),
                    merge: merge__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.MsgCreateClawbackVestingAccount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateClawbackVestingAccountResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("haqq.vesting.v1.MsgCreateClawbackVestingAccountResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateClawbackVestingAccountResponse {
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
            type Value = MsgCreateClawbackVestingAccountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.MsgCreateClawbackVestingAccountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCreateClawbackVestingAccountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCreateClawbackVestingAccountResponse {
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.MsgCreateClawbackVestingAccountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateVestingFunder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.funder_address.is_empty() {
            len += 1;
        }
        if !self.new_funder_address.is_empty() {
            len += 1;
        }
        if !self.vesting_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.MsgUpdateVestingFunder", len)?;
        if !self.funder_address.is_empty() {
            struct_ser.serialize_field("funderAddress", &self.funder_address)?;
        }
        if !self.new_funder_address.is_empty() {
            struct_ser.serialize_field("newFunderAddress", &self.new_funder_address)?;
        }
        if !self.vesting_address.is_empty() {
            struct_ser.serialize_field("vestingAddress", &self.vesting_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateVestingFunder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "funder_address",
            "funderAddress",
            "new_funder_address",
            "newFunderAddress",
            "vesting_address",
            "vestingAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FunderAddress,
            NewFunderAddress,
            VestingAddress,
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
                            "funderAddress" | "funder_address" => Ok(GeneratedField::FunderAddress),
                            "newFunderAddress" | "new_funder_address" => Ok(GeneratedField::NewFunderAddress),
                            "vestingAddress" | "vesting_address" => Ok(GeneratedField::VestingAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateVestingFunder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.MsgUpdateVestingFunder")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateVestingFunder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut funder_address__ = None;
                let mut new_funder_address__ = None;
                let mut vesting_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FunderAddress => {
                            if funder_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funderAddress"));
                            }
                            funder_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewFunderAddress => {
                            if new_funder_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newFunderAddress"));
                            }
                            new_funder_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VestingAddress => {
                            if vesting_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vestingAddress"));
                            }
                            vesting_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateVestingFunder {
                    funder_address: funder_address__.unwrap_or_default(),
                    new_funder_address: new_funder_address__.unwrap_or_default(),
                    vesting_address: vesting_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.MsgUpdateVestingFunder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateVestingFunderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("haqq.vesting.v1.MsgUpdateVestingFunderResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateVestingFunderResponse {
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
            type Value = MsgUpdateVestingFunderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.MsgUpdateVestingFunderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateVestingFunderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateVestingFunderResponse {
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.MsgUpdateVestingFunderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBalancesRequest {
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
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.QueryBalancesRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBalancesRequest {
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
            type Value = QueryBalancesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.QueryBalancesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBalancesRequest, V::Error>
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
                Ok(QueryBalancesRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.QueryBalancesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBalancesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.locked.is_empty() {
            len += 1;
        }
        if !self.unvested.is_empty() {
            len += 1;
        }
        if !self.vested.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.QueryBalancesResponse", len)?;
        if !self.locked.is_empty() {
            struct_ser.serialize_field("locked", &self.locked)?;
        }
        if !self.unvested.is_empty() {
            struct_ser.serialize_field("unvested", &self.unvested)?;
        }
        if !self.vested.is_empty() {
            struct_ser.serialize_field("vested", &self.vested)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBalancesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locked",
            "unvested",
            "vested",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locked,
            Unvested,
            Vested,
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
                            "locked" => Ok(GeneratedField::Locked),
                            "unvested" => Ok(GeneratedField::Unvested),
                            "vested" => Ok(GeneratedField::Vested),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBalancesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.QueryBalancesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBalancesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locked__ = None;
                let mut unvested__ = None;
                let mut vested__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locked => {
                            if locked__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locked"));
                            }
                            locked__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unvested => {
                            if unvested__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unvested"));
                            }
                            unvested__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vested => {
                            if vested__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vested"));
                            }
                            vested__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBalancesResponse {
                    locked: locked__.unwrap_or_default(),
                    unvested: unvested__.unwrap_or_default(),
                    vested: vested__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.QueryBalancesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryTotalLockedRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("haqq.vesting.v1.QueryTotalLockedRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryTotalLockedRequest {
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
            type Value = QueryTotalLockedRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.QueryTotalLockedRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryTotalLockedRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryTotalLockedRequest {
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.QueryTotalLockedRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryTotalLockedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.locked.is_empty() {
            len += 1;
        }
        if !self.unvested.is_empty() {
            len += 1;
        }
        if !self.vested.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("haqq.vesting.v1.QueryTotalLockedResponse", len)?;
        if !self.locked.is_empty() {
            struct_ser.serialize_field("locked", &self.locked)?;
        }
        if !self.unvested.is_empty() {
            struct_ser.serialize_field("unvested", &self.unvested)?;
        }
        if !self.vested.is_empty() {
            struct_ser.serialize_field("vested", &self.vested)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryTotalLockedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locked",
            "unvested",
            "vested",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locked,
            Unvested,
            Vested,
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
                            "locked" => Ok(GeneratedField::Locked),
                            "unvested" => Ok(GeneratedField::Unvested),
                            "vested" => Ok(GeneratedField::Vested),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryTotalLockedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct haqq.vesting.v1.QueryTotalLockedResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryTotalLockedResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locked__ = None;
                let mut unvested__ = None;
                let mut vested__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Locked => {
                            if locked__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locked"));
                            }
                            locked__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unvested => {
                            if unvested__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unvested"));
                            }
                            unvested__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vested => {
                            if vested__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vested"));
                            }
                            vested__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryTotalLockedResponse {
                    locked: locked__.unwrap_or_default(),
                    unvested: unvested__.unwrap_or_default(),
                    vested: vested__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("haqq.vesting.v1.QueryTotalLockedResponse", FIELDS, GeneratedVisitor)
    }
}
