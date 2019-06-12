# serde-piecewise-default
![Travis Build Status](https://img.shields.io/travis/com/boringcactus/serde-piecewise-default.svg) ![Crates.io](https://img.shields.io/crates/v/serde-piecewise-default.svg) ![docs.rs](https://docs.rs/serde-piecewise-default/badge.svg)

Uses serde's `Option<T>` handling to let you easily specify defaults for all fields at once
by implementing `Default`.

# Examples

```rust
use serde::Deserialize;
use serde_piecewise_default::DeserializePiecewiseDefault;

#[derive(DeserializePiecewiseDefault, PartialEq, Eq, Debug)]
struct Example {
    value: u8,
    enabled: bool,
}

impl Default for Example {
    fn default() -> Self {
        Example {
            value: 20,
            enabled: true,
        }
    }
}

let data: Example = serde_json::from_str(r#"{"value": 8}"#).unwrap();
assert_eq!(data, Example { value: 8, enabled: true });
let data: Example = serde_json::from_str(r#"{"enabled": false}"#).unwrap();
assert_eq!(data, Example { value: 20, enabled: false });
let data: Example = serde_json::from_str("{}").unwrap();
assert_eq!(data, Example { value: 20, enabled: true });
```

# Implementation Details

```rust
#[derive(DeserializePiecewiseDefault)]
struct Example {
    item1: i8,
    item2: String,
}
```
will expand to
```rust
struct Example {
    item1: i8,
    item2: String,
}

#[derive(Deserialize)]
struct OptionExample {
    item1: Option<i8>,
    item2: Option<String>,
}

impl<'de> Deserialize<'de> for Example {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        <OptionExample as Deserialize>::deserialize(deserializer)
            .map(|raw_result| {
                let OptionExample { item1, item2 } = raw_result;
                let default = <Example as Default>::default();
                let item1 = item1.unwrap_or(default.item1);
                let item2 = item2.unwrap_or(default.item2);
                Example { item1, item2 }
            })
    }
}
```
