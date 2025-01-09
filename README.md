# Serde with Value Affix

Based on [serde](https://docs.rs/serde/latest) and applies a prefix / suffix to the **value** of a field during serialization.
Note the difference vs `serde_with` macros: [with_prefix!](https://docs.rs/serde_with/latest/serde_with/macro.with_prefix.html) and [with_suffix!](https://docs.rs/serde_with/latest/serde_with/macro.with_suffix.html),
which apply to each field name of a struct instead.   

---

## Installation   

```toml
[dependencies]
serde_with_value_affix = "0.1.0"
```

## Examples

### Parsing JSON with suffix

```rust
use serde::{Deserialize, Serialize};
use serde_with_value_affix::with_affix;

struct MyStruct {
    #[serde(with = "value_prefix_a")]
    code: u8,
    #[serde(with = "value_suffix_celsius")]
    temperature: f32,
}

with_affix!(value_suffix_meter Prefix "A");
with_affix!(value_suffix_celsius Suffix "C");

// Serializes
MyStruct {
    code: 12,
    temperature: -12.3,
};

// into
{
  "length": "A12",
  "temperature": "-12.3C"
}
```


