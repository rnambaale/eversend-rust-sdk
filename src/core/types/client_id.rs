use std::fmt::Display;

/// A Client ID as shown in the Eversend business dashboard.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientId(String);

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ClientId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ClientId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
