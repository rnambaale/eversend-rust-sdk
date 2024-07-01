//! A module for interacting with account within Eversend.
//!

mod types;
mod operations;

pub use operations::*;

use crate::Eversend;

/// Account.
///
/// [Eversend Docs: Account Guide](https://eversend.readme.io/reference/get-account-profile)
pub struct Accounts<'a> {
    eversend: &'a Eversend,
}

impl<'a> Accounts<'a> {
    pub fn new(eversend: &'a Eversend) -> Self {
        Self { eversend }
    }
}
