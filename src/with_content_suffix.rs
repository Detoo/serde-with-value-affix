#[macro_export]
macro_rules! with_content_suffix {
    ($module:ident $suffix:expr) => {
        mod $module {
            use std::str::FromStr;
            use serde::{Deserializer, Serializer};
            use serde::de::{Error, Unexpected};

            pub fn serialize<S, T>(s: &T, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                T: ToString,
            {
                serializer.serialize_str(format!("{}{}", s.to_string(), $suffix).as_str())
            }

            pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
            where
                D: Deserializer<'de>,
                T: FromStr,
            {
                let s: String = d.deserialize_str(Visitor)?.parse().unwrap();
                let suffix_len = $suffix.len();
                let s_len = s.len();
                if (s_len >= suffix_len && &s[(s_len - suffix_len)..] == $suffix) {
                    match s[..s.len()-$suffix.len()].parse::<T>() {
                        Ok(v) => Ok(v),
                        Err(_) => Err(Error::invalid_value(Unexpected::Str(&s), &"to be parsable")),
                    }
                } else {
                    Err(Error::invalid_value(Unexpected::Str(&s), &"string with a specific suffix"))
                }
            }

            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = String;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a string")
                }

                fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(s.to_string())
                }
            }

            // TODO Deprecated
            // impl<'de> serde::de::Visitor<'de> for Visitor {
            //     type Value = u8;
            //
            //     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            //         formatter.write_str("a parsable value after removing the suffix")
            //     }
            //
            //     fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            //     where
            //         E: serde::de::Error,
            //     {
            //         let suffix_len = $suffix.len();
            //         let s_len = s.len();
            //         if (s_len >= suffix_len && &s[(s_len - suffix_len)..] == $suffix) {
            //             Ok(s[..s.len()-$suffix.len()].parse().unwrap())
            //         } else {
            //             Err(serde::de::Error::invalid_value(Unexpected::Str(s), &"string with a specific suffix"))
            //         }
            //     }
            // }
        }
    };
}