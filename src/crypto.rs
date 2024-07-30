mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::Eversend;

/// Crypto.
///
/// [Eversend Docs: Crypto Guide](https://eversend.readme.io/reference/fetch-asset-chains)
pub struct Crypto<'a> {
    pub eversend: &'a Eversend,
}

impl<'a> Crypto<'a> {
    pub fn new(eversend: &'a Eversend) -> Self {
        Self {
            eversend,
        }
    }
}
