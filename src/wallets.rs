//! A module for interacting with the Eversend Wallets API.
//!
//! [Eversend Docs: Wallets Guide](https://eversend.readme.io/reference/get-wallets)


mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::Eversend;

/// Wallets.
///
/// [Eversend Docs: Wallets Guide](https://eversend.readme.io/reference/get-wallets)
pub struct Wallets<'a> {
    eversend: &'a Eversend,
}

impl<'a> Wallets<'a> {
    pub fn new(eversend: &'a Eversend) -> Self {
        Self { eversend }
    }
}
