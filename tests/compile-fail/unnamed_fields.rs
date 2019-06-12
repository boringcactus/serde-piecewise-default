use serde_piecewise_default::*;
use serde::{Deserialize, Deserializer};

extern crate serde_json;
extern crate serde_piecewise_default;

#[derive(DeserializePiecewiseDefault)] //~  ERROR proc-macro derive panicked
                                       //~| help: message: can only use piecewise default with named fields
struct Example(i8, bool);

impl Default for Example {
    fn default() -> Self {
        Example(-7, true)
    }
}

fn main() {
    let data = "[-34, false]";
    let data: Example = serde_json::from_str(data).unwrap();
    assert_eq!(-34, data.0);
    assert_eq!(false, data.1);
}
