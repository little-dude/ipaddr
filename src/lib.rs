#![cfg_attr(feature = "nightly", feature(tool_attributes))]
#![cfg_attr(feature = "nightly", feature(custom_attribute))]
#![cfg_attr(feature = "nightly", allow(unused_attributes))]
#![cfg_attr(feature = "cargo-clippy", allow(module_inception))]
#![feature(try_from)]
#![feature(range_contains)]

extern crate core;

mod errors;
pub use self::errors::*;

mod ipv4;
pub use self::ipv4::*;
mod ipv6;
pub use self::ipv6::*;
