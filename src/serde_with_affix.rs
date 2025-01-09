#[macro_export]
macro_rules! with_affix {
    ($module:ident $affix_type:ident $affix:expr) => {
        mod $module {
            use std::fmt::Display;
            use std::str::FromStr;
            use serde::{Deserializer, Serializer, Deserialize};
            use serde::de::{Error, Unexpected};

            #[allow(dead_code)] // By design, only one of the variants will be used per construction
            pub enum AffixType {
                Prefix,
                Suffix,
            }
            
            pub fn serialize<S, T>(s: &T, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                T: ToString,
            {
                match AffixType::$affix_type {
                    AffixType::Prefix => serializer.serialize_str(format!("{}{}", $affix, s.to_string()).as_str()),
                    AffixType::Suffix => serializer.serialize_str(format!("{}{}", s.to_string(), $affix).as_str()),
                }
            }

            pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
            where
                D: Deserializer<'de>,
                T: FromStr,
                T::Err: Display,
            {
                let s: String = String::deserialize(d)?.parse().unwrap();

                let parts: Vec<_> = s.split($affix).collect();
                if parts.len() > 1 {
                    let s_removed_affix = match AffixType::$affix_type {
                        AffixType::Prefix => parts[1..].join(""),
                        AffixType::Suffix => parts[..(parts.len() - 1)].join(""),
                    };
                    match s_removed_affix.parse() {
                        Ok(v) => Ok(v),
                        Err(_) => Err(D::Error::invalid_value(Unexpected::Str(&s_removed_affix), &"string parsable to the native type"))
                    }

                } else {
                    Err(D::Error::invalid_value(Unexpected::Str(&s), &"string with a proper affix"))
                }
            }
        }
    };
}