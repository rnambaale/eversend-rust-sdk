use std::fmt::Display;

use serde::Deserialize;

/// An API token to authenticate with the Eversend API.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct ApiToken(String);

impl Display for ApiToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ApiToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ApiToken {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
