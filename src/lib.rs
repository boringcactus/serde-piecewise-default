//! Uses serde's `Option<T>` handling to let you easily specify defaults for all fields at once
//! by implementing `Default`.
//!
//! # Panics
//!
//! Will panic at compile time if you use `#[derive(DeserializePiecewiseDefault)]` on a struct
//! with unnamed fields, a struct that does not implement `Default`, or an enum.
//!
//! # Examples
//!
//! ```
//! use serde::Deserialize;
//! use serde_piecewise_default::DeserializePiecewiseDefault;
//!
//! #[derive(DeserializePiecewiseDefault, PartialEq, Eq, Debug)]
//! struct Example {
//!     value: u8,
//!     enabled: bool,
//! }
//!
//! impl Default for Example {
//!     fn default() -> Self {
//!         Example {
//!             value: 20,
//!             enabled: true,
//!         }
//!     }
//! }
//!
//! let data: Example = serde_json::from_str(r#"{"value": 8}"#).unwrap();
//! assert_eq!(data, Example { value: 8, enabled: true });
//! let data: Example = serde_json::from_str(r#"{"enabled": false}"#).unwrap();
//! assert_eq!(data, Example { value: 20, enabled: false });
//! let data: Example = serde_json::from_str("{}").unwrap();
//! assert_eq!(data, Example { value: 20, enabled: true });
//! ```

extern crate serde;
extern crate serde_piecewise_default_derive;

pub use serde_piecewise_default_derive::DeserializePiecewiseDefault;
