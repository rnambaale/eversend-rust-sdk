mod operations;
mod types;

pub use operations::*;
pub use types::*;

use crate::Eversend;

pub struct Payouts<'a> {
    eversend: &'a Eversend,
}

impl<'a> Payouts<'a> {
    pub fn new(eversend: &'a Eversend) -> Self {
        Self { eversend }
    }
}
