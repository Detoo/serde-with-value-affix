# Serde with Value Affix &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Rustc 1.78+]][rustc]

[Build Status]: https://img.shields.io/github/actions/workflow/status/Detoo/serde-with-value-affix/ci.yml?branch=master
[actions]: https://github.com/Detoo/serde-with-value-affix/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/serde_with_value_affix.svg
[crates.io]: https://crates.io/crates/serde_with_value_affix
[Rustc 1.78+]: https://img.shields.io/badge/rustc-1.78+-lightgray.svg
[rustc]: https://blog.rust-lang.org/2024/05/02/Rust-1.78.0.html

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


