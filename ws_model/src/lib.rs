extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;

/// primitive data store

pub mod site;
pub mod types;

/// tiny convenient module
pub mod prelude {
    pub use super::site::Site;
    pub use super::types::*;
}