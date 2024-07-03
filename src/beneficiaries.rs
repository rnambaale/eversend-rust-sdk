mod types;
mod operations;

pub use types::*;
pub use operations::*;

use crate::Eversend;

/// Beneficiaries.
///
/// [Eversend Docs: Beneficiaries Guide](https://eversend.readme.io/reference/create-beneficiaries)
pub struct Beneficiaries<'a> {
    eversend: &'a Eversend
}

impl<'a> Beneficiaries<'a> {
    pub fn new(eversend: &'a Eversend) -> Self {
        Self { eversend }
    }
}
