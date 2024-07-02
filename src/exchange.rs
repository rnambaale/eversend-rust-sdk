//! A module for interacting with exchanges within Eversend.
//!

mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::Eversend;

/// Exchange.
///
/// [Eversend Docs: Exchange Guide](https://eversend.readme.io/reference/create-quotation)
pub struct Exchange<'a> {
    pub eversend: &'a Eversend
}

impl<'a> Exchange<'a> {
    pub fn new(eversend: &'a Eversend) -> Self {
        Self { eversend }
    }
}
