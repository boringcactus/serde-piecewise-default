use serde_piecewise_default::*;
use serde::{Deserialize, Deserializer};

extern crate serde;
extern crate serde_json;
extern crate serde_piecewise_default;

#[derive(DeserializePiecewiseDefault)] //~ ERROR the trait bound `Example: std::default::Default` is not satisfied [E0277]
struct Example {
    datum: bool,
}

fn main() {
    let data = r#"{"datum": false}"#;
    let data: Example = serde_json::from_str(data).unwrap();
    assert_eq!(false, data.datum);
}
