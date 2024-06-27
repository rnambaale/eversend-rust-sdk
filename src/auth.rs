use crate::Eversend;

mod operations;

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
