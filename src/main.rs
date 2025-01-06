mod with_content_suffix;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct MyStruct {
    // So this does add a prefix to the content instead of the field name
    #[serde(with = "content_suffix_kek")]
    key: u8
}

with_content_suffix!(content_suffix_kek "_kek");

fn main() {
    let my_struct = MyStruct {
        key: 123,
    };

    let serialized = serde_json::to_string(&my_struct).unwrap();
    println!("serialized (json): {serialized}");

    let deserialized = serde_json::from_str::<MyStruct>(&serialized).unwrap();
    println!("deserialized: {:?}", deserialized);
}
