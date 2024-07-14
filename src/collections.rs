mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::Eversend;

/// Collections.
///
/// [Eversend Docs: Collections Guide](https://eversend.readme.io/reference/get-collection-fees)
pub struct Collections<'a> {
    eversend: &'a Eversend,
}

impl<'a> Collections<'a> {
    pub fn new(eversend: &'a Eversend) -> Self {
        Self { eversend }
    }
}
