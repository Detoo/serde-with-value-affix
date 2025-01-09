mod serde_with_affix;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct MyStruct {
    #[serde(with = "content_suffix_kek")]
    key1: u8,

    #[serde(with = "content_suffix_kek")]
    key2: i16,
}

with_affix!(content_suffix_kek Suffix "_kek");

fn main() {
    let my_struct = MyStruct {
        key1: 123,
        key2: 456,
    };

    let serialized = serde_json::to_string(&my_struct).unwrap();
    // let serialized = r#"{"key1":"123a_kek","key2":"456_kek"}"#;
    println!("serialized (json): {serialized}");

    let deserialized = serde_json::from_str::<MyStruct>(&serialized).unwrap();
    println!("deserialized: {:?}", deserialized);
}
