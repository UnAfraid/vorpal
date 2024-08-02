// @generated
impl serde::Serialize for ConfigPackageBuild {
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
        if !self.environment.is_empty() {
            len += 1;
        }
        if self.image.is_some() {
            len += 1;
        }
        if !self.packages.is_empty() {
            len += 1;
        }
        if !self.script.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vorpal.config.v0.ConfigPackageBuild", len)?;
        if self.system != 0 {
            let v = super::super::common::v0::System::from_i32(self.system)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.system)))?;
            struct_ser.serialize_field("system", &v)?;
        }
        if !self.environment.is_empty() {
            struct_ser.serialize_field("environment", &self.environment)?;
        }
        if let Some(v) = self.image.as_ref() {
            struct_ser.serialize_field("image", v)?;
        }
        if !self.packages.is_empty() {
            struct_ser.serialize_field("packages", &self.packages)?;
        }
        if !self.script.is_empty() {
            struct_ser.serialize_field("script", &self.script)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigPackageBuild {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "system",
            "environment",
            "image",
            "packages",
            "script",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            System,
            Environment,
            Image,
            Packages,
            Script,
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
                            "environment" => Ok(GeneratedField::Environment),
                            "image" => Ok(GeneratedField::Image),
                            "packages" => Ok(GeneratedField::Packages),
                            "script" => Ok(GeneratedField::Script),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigPackageBuild;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vorpal.config.v0.ConfigPackageBuild")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfigPackageBuild, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut system__ = None;
                let mut environment__ = None;
                let mut image__ = None;
                let mut packages__ = None;
                let mut script__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::System => {
                            if system__.is_some() {
                                return Err(serde::de::Error::duplicate_field("system"));
                            }
                            system__ = Some(map.next_value::<super::super::common::v0::System>()? as i32);
                        }
                        GeneratedField::Environment => {
                            if environment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("environment"));
                            }
                            environment__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Image => {
                            if image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image__ = map.next_value()?;
                        }
                        GeneratedField::Packages => {
                            if packages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packages"));
                            }
                            packages__ = Some(map.next_value()?);
                        }
                        GeneratedField::Script => {
                            if script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            script__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ConfigPackageBuild {
                    system: system__.unwrap_or_default(),
                    environment: environment__.unwrap_or_default(),
                    image: image__,
                    packages: packages__.unwrap_or_default(),
                    script: script__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vorpal.config.v0.ConfigPackageBuild", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigPackageOutput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vorpal.config.v0.ConfigPackageOutput", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigPackageOutput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Hash,
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
                            "name" => Ok(GeneratedField::Name),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigPackageOutput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vorpal.config.v0.ConfigPackageOutput")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfigPackageOutput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ConfigPackageOutput {
                    name: name__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vorpal.config.v0.ConfigPackageOutput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigPackageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.build.is_some() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vorpal.config.v0.ConfigPackageRequest", len)?;
        if let Some(v) = self.build.as_ref() {
            struct_ser.serialize_field("build", v)?;
        }
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigPackageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "build",
            "source",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Build,
            Source,
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
                            "build" => Ok(GeneratedField::Build),
                            "source" => Ok(GeneratedField::Source),
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
            type Value = ConfigPackageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vorpal.config.v0.ConfigPackageRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfigPackageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut build__ = None;
                let mut source__ = None;
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Build => {
                            if build__.is_some() {
                                return Err(serde::de::Error::duplicate_field("build"));
                            }
                            build__ = map.next_value()?;
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = map.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ConfigPackageRequest {
                    build: build__,
                    source: source__,
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vorpal.config.v0.ConfigPackageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigPackageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.package_output.is_some() {
            len += 1;
        }
        if !self.log_output.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vorpal.config.v0.ConfigPackageResponse", len)?;
        if let Some(v) = self.package_output.as_ref() {
            struct_ser.serialize_field("packageOutput", v)?;
        }
        if !self.log_output.is_empty() {
            struct_ser.serialize_field("logOutput", &self.log_output)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigPackageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "package_output",
            "packageOutput",
            "log_output",
            "logOutput",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PackageOutput,
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
                            "packageOutput" | "package_output" => Ok(GeneratedField::PackageOutput),
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
            type Value = ConfigPackageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vorpal.config.v0.ConfigPackageResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfigPackageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut package_output__ = None;
                let mut log_output__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PackageOutput => {
                            if package_output__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageOutput"));
                            }
                            package_output__ = map.next_value()?;
                        }
                        GeneratedField::LogOutput => {
                            if log_output__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logOutput"));
                            }
                            log_output__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ConfigPackageResponse {
                    package_output: package_output__,
                    log_output: log_output__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vorpal.config.v0.ConfigPackageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigPackageSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind != 0 {
            len += 1;
        }
        if self.hash.is_some() {
            len += 1;
        }
        if !self.ignore_paths.is_empty() {
            len += 1;
        }
        if !self.uri.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vorpal.config.v0.ConfigPackageSource", len)?;
        if self.kind != 0 {
            let v = ConfigPackageSourceKind::from_i32(self.kind)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if let Some(v) = self.hash.as_ref() {
            struct_ser.serialize_field("hash", v)?;
        }
        if !self.ignore_paths.is_empty() {
            struct_ser.serialize_field("ignorePaths", &self.ignore_paths)?;
        }
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigPackageSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "hash",
            "ignore_paths",
            "ignorePaths",
            "uri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Hash,
            IgnorePaths,
            Uri,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "hash" => Ok(GeneratedField::Hash),
                            "ignorePaths" | "ignore_paths" => Ok(GeneratedField::IgnorePaths),
                            "uri" => Ok(GeneratedField::Uri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigPackageSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vorpal.config.v0.ConfigPackageSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfigPackageSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut hash__ = None;
                let mut ignore_paths__ = None;
                let mut uri__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map.next_value::<ConfigPackageSourceKind>()? as i32);
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = map.next_value()?;
                        }
                        GeneratedField::IgnorePaths => {
                            if ignore_paths__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignorePaths"));
                            }
                            ignore_paths__ = Some(map.next_value()?);
                        }
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ConfigPackageSource {
                    kind: kind__.unwrap_or_default(),
                    hash: hash__,
                    ignore_paths: ignore_paths__.unwrap_or_default(),
                    uri: uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vorpal.config.v0.ConfigPackageSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigPackageSourceKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UnknownSource => "UNKNOWN_SOURCE",
            Self::Local => "LOCAL",
            Self::Http => "HTTP",
            Self::Git => "GIT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ConfigPackageSourceKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN_SOURCE",
            "LOCAL",
            "HTTP",
            "GIT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigPackageSourceKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ConfigPackageSourceKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ConfigPackageSourceKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN_SOURCE" => Ok(ConfigPackageSourceKind::UnknownSource),
                    "LOCAL" => Ok(ConfigPackageSourceKind::Local),
                    "HTTP" => Ok(ConfigPackageSourceKind::Http),
                    "GIT" => Ok(ConfigPackageSourceKind::Git),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
