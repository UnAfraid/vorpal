// @generated
impl serde::Serialize for PackageBuildRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.system != 0 {
            len += 1;
        }
        if !self.build_environment.is_empty() {
            len += 1;
        }
        if self.build_image.is_some() {
            len += 1;
        }
        if !self.build_packages.is_empty() {
            len += 1;
        }
        if !self.build_script.is_empty() {
            len += 1;
        }
        if !self.source_hash.is_empty() {
            len += 1;
        }
        if !self.source_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vorpal.package.v0.PackageBuildRequest", len)?;
        if self.system != 0 {
            let v = super::super::common::v0::System::from_i32(self.system)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.system)))?;
            struct_ser.serialize_field("system", &v)?;
        }
        if !self.build_environment.is_empty() {
            struct_ser.serialize_field("buildEnvironment", &self.build_environment)?;
        }
        if let Some(v) = self.build_image.as_ref() {
            struct_ser.serialize_field("buildImage", v)?;
        }
        if !self.build_packages.is_empty() {
            struct_ser.serialize_field("buildPackages", &self.build_packages)?;
        }
        if !self.build_script.is_empty() {
            struct_ser.serialize_field("buildScript", &self.build_script)?;
        }
        if !self.source_hash.is_empty() {
            struct_ser.serialize_field("sourceHash", &self.source_hash)?;
        }
        if !self.source_name.is_empty() {
            struct_ser.serialize_field("sourceName", &self.source_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PackageBuildRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "system",
            "build_environment",
            "buildEnvironment",
            "build_image",
            "buildImage",
            "build_packages",
            "buildPackages",
            "build_script",
            "buildScript",
            "source_hash",
            "sourceHash",
            "source_name",
            "sourceName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            System,
            BuildEnvironment,
            BuildImage,
            BuildPackages,
            BuildScript,
            SourceHash,
            SourceName,
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
                            "system" => Ok(GeneratedField::System),
                            "buildEnvironment" | "build_environment" => Ok(GeneratedField::BuildEnvironment),
                            "buildImage" | "build_image" => Ok(GeneratedField::BuildImage),
                            "buildPackages" | "build_packages" => Ok(GeneratedField::BuildPackages),
                            "buildScript" | "build_script" => Ok(GeneratedField::BuildScript),
                            "sourceHash" | "source_hash" => Ok(GeneratedField::SourceHash),
                            "sourceName" | "source_name" => Ok(GeneratedField::SourceName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PackageBuildRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vorpal.package.v0.PackageBuildRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PackageBuildRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut system__ = None;
                let mut build_environment__ = None;
                let mut build_image__ = None;
                let mut build_packages__ = None;
                let mut build_script__ = None;
                let mut source_hash__ = None;
                let mut source_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::System => {
                            if system__.is_some() {
                                return Err(serde::de::Error::duplicate_field("system"));
                            }
                            system__ = Some(map.next_value::<super::super::common::v0::System>()? as i32);
                        }
                        GeneratedField::BuildEnvironment => {
                            if build_environment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buildEnvironment"));
                            }
                            build_environment__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::BuildImage => {
                            if build_image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buildImage"));
                            }
                            build_image__ = map.next_value()?;
                        }
                        GeneratedField::BuildPackages => {
                            if build_packages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buildPackages"));
                            }
                            build_packages__ = Some(map.next_value()?);
                        }
                        GeneratedField::BuildScript => {
                            if build_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buildScript"));
                            }
                            build_script__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceHash => {
                            if source_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceHash"));
                            }
                            source_hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceName => {
                            if source_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceName"));
                            }
                            source_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PackageBuildRequest {
                    system: system__.unwrap_or_default(),
                    build_environment: build_environment__.unwrap_or_default(),
                    build_image: build_image__,
                    build_packages: build_packages__.unwrap_or_default(),
                    build_script: build_script__.unwrap_or_default(),
                    source_hash: source_hash__.unwrap_or_default(),
                    source_name: source_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vorpal.package.v0.PackageBuildRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PackageBuildResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_output.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vorpal.package.v0.PackageBuildResponse", len)?;
        if !self.log_output.is_empty() {
            struct_ser.serialize_field("logOutput", &self.log_output)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PackageBuildResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_output",
            "logOutput",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogOutput,
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
                            "logOutput" | "log_output" => Ok(GeneratedField::LogOutput),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PackageBuildResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vorpal.package.v0.PackageBuildResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PackageBuildResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_output__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LogOutput => {
                            if log_output__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logOutput"));
                            }
                            log_output__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PackageBuildResponse {
                    log_output: log_output__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vorpal.package.v0.PackageBuildResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PackagePrepareRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.source_data.is_empty() {
            len += 1;
        }
        if !self.source_hash.is_empty() {
            len += 1;
        }
        if !self.source_name.is_empty() {
            len += 1;
        }
        if !self.source_signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vorpal.package.v0.PackagePrepareRequest", len)?;
        if !self.source_data.is_empty() {
            struct_ser.serialize_field("sourceData", pbjson::private::base64::encode(&self.source_data).as_str())?;
        }
        if !self.source_hash.is_empty() {
            struct_ser.serialize_field("sourceHash", &self.source_hash)?;
        }
        if !self.source_name.is_empty() {
            struct_ser.serialize_field("sourceName", &self.source_name)?;
        }
        if !self.source_signature.is_empty() {
            struct_ser.serialize_field("sourceSignature", &self.source_signature)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PackagePrepareRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_data",
            "sourceData",
            "source_hash",
            "sourceHash",
            "source_name",
            "sourceName",
            "source_signature",
            "sourceSignature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceData,
            SourceHash,
            SourceName,
            SourceSignature,
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
                            "sourceData" | "source_data" => Ok(GeneratedField::SourceData),
                            "sourceHash" | "source_hash" => Ok(GeneratedField::SourceHash),
                            "sourceName" | "source_name" => Ok(GeneratedField::SourceName),
                            "sourceSignature" | "source_signature" => Ok(GeneratedField::SourceSignature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PackagePrepareRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vorpal.package.v0.PackagePrepareRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PackagePrepareRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_data__ = None;
                let mut source_hash__ = None;
                let mut source_name__ = None;
                let mut source_signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourceData => {
                            if source_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceData"));
                            }
                            source_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SourceHash => {
                            if source_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceHash"));
                            }
                            source_hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceName => {
                            if source_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceName"));
                            }
                            source_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceSignature => {
                            if source_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceSignature"));
                            }
                            source_signature__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PackagePrepareRequest {
                    source_data: source_data__.unwrap_or_default(),
                    source_hash: source_hash__.unwrap_or_default(),
                    source_name: source_name__.unwrap_or_default(),
                    source_signature: source_signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vorpal.package.v0.PackagePrepareRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PackagePrepareResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_output.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vorpal.package.v0.PackagePrepareResponse", len)?;
        if !self.log_output.is_empty() {
            struct_ser.serialize_field("logOutput", &self.log_output)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PackagePrepareResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_output",
            "logOutput",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogOutput,
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
                            "logOutput" | "log_output" => Ok(GeneratedField::LogOutput),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PackagePrepareResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vorpal.package.v0.PackagePrepareResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PackagePrepareResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_output__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LogOutput => {
                            if log_output__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logOutput"));
                            }
                            log_output__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PackagePrepareResponse {
                    log_output: log_output__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vorpal.package.v0.PackagePrepareResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrepareBuildPackage {
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
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vorpal.package.v0.PrepareBuildPackage", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PrepareBuildPackage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Name,
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
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PrepareBuildPackage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vorpal.package.v0.PrepareBuildPackage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PrepareBuildPackage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PrepareBuildPackage {
                    hash: hash__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vorpal.package.v0.PrepareBuildPackage", FIELDS, GeneratedVisitor)
    }
}
