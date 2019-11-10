#![deny(missing_docs)]
//! A simple key/value store.

#[macro_use]
extern crate log;

pub use server::SidecarServer;
pub use result::{LKError, Result};

mod server;
mod result;
