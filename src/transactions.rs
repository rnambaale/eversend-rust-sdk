mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::Eversend;

/// Transactions.
///
/// [Eversend Docs: Transactions Guide](https://eversend.readme.io/reference/get-transactions)
pub struct Transactions<'a> {
    pub eversend: &'a Eversend
}

impl<'a> Transactions<'a> {
    pub fn new(eversend: &'a Eversend) -> Self {
        Self { eversend }
    }
}
