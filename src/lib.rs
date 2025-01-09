//! # Serde with Value Affix
//!
//! Based on [serde](https://docs.rs/serde/latest) and applies a prefix / suffix to the **value** of a field during serialization.
//! Note the difference vs `serde_with` macros: [with_prefix!](https://docs.rs/serde_with/latest/serde_with/macro.with_prefix.html) and [with_suffix!](https://docs.rs/serde_with/latest/serde_with/macro.with_suffix.html),
//! which apply to each field name of a struct instead.
//!
//! ## Example: Parsing JSON with suffix
//!
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_with_value_affix::with_affix;
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "value_prefix_a")]
//!     code: u8,
//!     #[serde(with = "value_suffix_celsius")]
//!     temperature: f32,
//! }
//!
//! with_affix!(value_prefix_a Prefix "A");
//! with_affix!(value_suffix_celsius Suffix "C");
//!
//! // Serializes
//! MyStruct {
//!     code: 12,
//!     temperature: -12.3,
//! };
//!
//! // into
//! # let json = r#"
//! {
//!   "length": "A12",
//!   "temperature": "-12.3C"
//! }
//! # "#;
//! ```

/// This macro implements the adapter to use with `#[serde(with = )]` that
/// serializes with an added affix on the field value and deserialize by trimming away that affix.
///
/// You can set the affix type with the keyword `Prefix` or `Suffix`.
/// For example, `with_affix!(value_suffix_celsius Suffix "C");` creates the adapter `value_suffix_celsius`
/// which adds suffix "C" to a field's serialized value when assigned to the field:
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use serde_with_value_affix::with_affix;
/// #[derive(Serialize, Deserialize)]
/// struct MyStruct {
///     #[serde(with = "value_suffix_celsius")]
///     temperature: f32,
/// }
/// # with_affix!(value_suffix_celsius Suffix "C");
/// ```
#[macro_export]
macro_rules! with_affix {
    ($module:ident $affix_type:ident $affix:expr) => {
        mod $module {
            use std::fmt::Display;
            use std::str::FromStr;
            use serde::{Deserializer, Serializer, Deserialize};
            use serde::de::{Error, Unexpected};

            #[allow(dead_code)] // By design, only one of the variants will be used per construction
            enum AffixType {
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

#[cfg(test)]
mod tests_prefix {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyStruct {
        #[serde(with = "value_prefix_a")]
        code_a: u8,

        #[serde(with = "value_prefix_b")]
        code_b: u8,
    }

    with_affix!(value_prefix_a Prefix "A");
    with_affix!(value_prefix_b Prefix "B");

    #[test]
    fn test_normal() {
        let my_struct = MyStruct {
            code_a: 12,
            code_b: 34,
        };

        let serialized = serde_json::to_string(&my_struct).unwrap();
        assert_eq!(serialized, r#"{"code_a":"A12","code_b":"B34"}"#);

        let deserialized = serde_json::from_str::<MyStruct>(&serialized).unwrap();
        assert_eq!(deserialized, my_struct);
    }

    #[test]
    fn test_deserialize_error_missing_affix() {
        let error = serde_json::from_str::<MyStruct>(r#"{"code_a":"12","code_b":"B34"}"#).unwrap_err();
        assert_eq!(error.to_string(), r#"invalid value: string "12", expected string with a proper affix at line 1 column 14"#);
    }

    #[test]
    fn test_deserialize_error_parsing() {
        let error = serde_json::from_str::<MyStruct>(r#"{"code_a":"A12","code_b":"B34u"}"#).unwrap_err();
        assert_eq!(error.to_string(), r#"invalid value: string "34u", expected string parsable to the native type at line 1 column 32"#);
    }
}

#[cfg(test)]
mod tests_suffix {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyStruct {
        #[serde(with = "value_suffix_meter")]
        length: u16,

        #[serde(with = "value_suffix_celsius")]
        temperature: f32,
    }

    with_affix!(value_suffix_meter Suffix "m");
    with_affix!(value_suffix_celsius Suffix "C");

    #[test]
    fn test_normal() {
        let my_struct = MyStruct {
            length: 123,
            temperature: -12.3,
        };

        let serialized = serde_json::to_string(&my_struct).unwrap();
        assert_eq!(serialized, r#"{"length":"123m","temperature":"-12.3C"}"#);

        let deserialized = serde_json::from_str::<MyStruct>(&serialized).unwrap();
        assert_eq!(deserialized, my_struct);
    }

    #[test]
    fn test_deserialize_error_missing_affix() {
        let error = serde_json::from_str::<MyStruct>(r#"{"length":"123","temperature":"-12.3C"}"#).unwrap_err();
        assert_eq!(error.to_string(), r#"invalid value: string "123", expected string with a proper affix at line 1 column 15"#);
    }

    #[test]
    fn test_deserialize_error_parsing() {
        let error = serde_json::from_str::<MyStruct>(r#"{"length":"123m","temperature":"-12.3fC"}"#).unwrap_err();
        assert_eq!(error.to_string(), r#"invalid value: string "-12.3f", expected string parsable to the native type at line 1 column 41"#);
    }
}
