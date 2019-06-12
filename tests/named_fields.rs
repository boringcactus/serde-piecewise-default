use serde_piecewise_default::*;
use serde::Deserialize;

extern crate serde_json;

#[derive(DeserializePiecewiseDefault)]
struct Example {
    data1: u8,
    data2: bool,
    data3: String,
}

impl Default for Example {
    fn default() -> Self {
        Example {
            data1: 7,
            data2: false,
            data3: "Hello".to_owned()
        }
    }
}

#[test]
fn full() {
    let data = r#"
        {
            "data1": 8,
            "data2": true,
            "data3": "Howdy"
        }
    "#;
    let data: Example = serde_json::from_str(data).unwrap();
    assert_eq!(8, data.data1);
    assert_eq!(true, data.data2);
    assert_eq!("Howdy", data.data3);
}

#[test]
fn partial() {
    let data = r#"
        {
            "data3": "Ahoy"
        }
    "#;
    let data: Example = serde_json::from_str(data).unwrap();
    assert_eq!(7, data.data1);
    assert_eq!(false, data.data2);
    assert_eq!("Ahoy", data.data3);
}

#[test]
fn empty() {
    let data = "{}";
    let data: Example = serde_json::from_str(data).unwrap();
    assert_eq!(7, data.data1);
    assert_eq!(false, data.data2);
    assert_eq!("Hello", data.data3);
}
