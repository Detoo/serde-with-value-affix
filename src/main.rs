use serde::{Deserialize, Serialize};
use serde_with::with_prefix;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct MyStruct {
    // So this does add a prefix to the content instead of the field name
    #[serde(with = "prefix_kek")]
    key: String
}

with_prefix!(prefix_kek "kek_");

fn main() {
    let my_struct = MyStruct {
        key: "test".to_string(),
    };

    let serialized = serde_json::to_string(&my_struct).unwrap();
    println!("serialized (json): {serialized}");

    let deserialized = serde_json::from_str::<MyStruct>(&serialized).unwrap();
    println!("deserialized: {:?}", deserialized);
}
