//! A module for interacting with authentication within Eversend.
//!

mod operations;

pub use operations::*;

use crate::Eversend;

/// Authentication.
///
pub struct Auth<'a> {
    eversend: &'a Eversend,
}

impl<'a> Auth<'a> {
    /// Returns a new [`Auth`] instance for the provided Eversend client.
    pub fn new(eversend: &'a Eversend) -> Self {
        Self { eversend }
    }
}
