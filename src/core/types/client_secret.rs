use std::fmt::Display;

/// A Client Secret as shown in the Eversend business dashboard.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientSecret(String);

impl Display for ClientSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ClientSecret {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ClientSecret {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
