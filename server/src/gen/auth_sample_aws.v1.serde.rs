// @generated
impl serde::Serialize for SignUpRequest {
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
        if !self.email.is_empty() {
            len += 1;
        }
        if self.birthday != 0 {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        if !self.password_confirm.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth_sample_aws.v1.SignUpRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if self.birthday != 0 {
            struct_ser.serialize_field("birthday", ToString::to_string(&self.birthday).as_str())?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        if !self.password_confirm.is_empty() {
            struct_ser.serialize_field("passwordConfirm", &self.password_confirm)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignUpRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "email",
            "birthday",
            "password",
            "password_confirm",
            "passwordConfirm",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Email,
            Birthday,
            Password,
            PasswordConfirm,
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
                            "email" => Ok(GeneratedField::Email),
                            "birthday" => Ok(GeneratedField::Birthday),
                            "password" => Ok(GeneratedField::Password),
                            "passwordConfirm" | "password_confirm" => Ok(GeneratedField::PasswordConfirm),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignUpRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth_sample_aws.v1.SignUpRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignUpRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut email__ = None;
                let mut birthday__ = None;
                let mut password__ = None;
                let mut password_confirm__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map.next_value()?);
                        }
                        GeneratedField::Birthday => {
                            if birthday__.is_some() {
                                return Err(serde::de::Error::duplicate_field("birthday"));
                            }
                            birthday__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map.next_value()?);
                        }
                        GeneratedField::PasswordConfirm => {
                            if password_confirm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordConfirm"));
                            }
                            password_confirm__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SignUpRequest {
                    name: name__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                    birthday: birthday__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                    password_confirm: password_confirm__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth_sample_aws.v1.SignUpRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignUpResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("auth_sample_aws.v1.SignUpResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignUpResponse {
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
            type Value = SignUpResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth_sample_aws.v1.SignUpResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignUpResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SignUpResponse {
                })
            }
        }
        deserializer.deserialize_struct("auth_sample_aws.v1.SignUpResponse", FIELDS, GeneratedVisitor)
    }
}
