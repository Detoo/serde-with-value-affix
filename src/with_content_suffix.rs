#[macro_export]
macro_rules! with_content_suffix {
    ($module:ident $suffix:expr) => {
        mod $module {
            use serde::{Deserializer, Serializer};
            use serde::de::Unexpected;

            pub fn serialize<S>(s: &u8, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(format!("{s}{}", $suffix).as_str())
            }

            pub fn deserialize<'de, D>(d: D) -> Result<u8, D::Error>
            where
                D: Deserializer<'de>,
            {
                d.deserialize_str(Visitor)
            }

            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = u8;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("an unsigned integer between 0 and 2^8")
                }

                fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    let suffix_len = $suffix.len();
                    let s_len = s.len();
                    if (s_len >= suffix_len && &s[(s_len - suffix_len)..] == $suffix) {
                        Ok(s[..s.len()-$suffix.len()].parse().unwrap())
                    } else {
                        Err(serde::de::Error::invalid_value(Unexpected::Str(s), &"string with a specific suffix"))
                    }
                }
            }
        }
    };
}