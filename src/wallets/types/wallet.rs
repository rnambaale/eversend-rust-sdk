use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wallet {
    /// The currency of the wallet.
    pub currency: String,

    #[serde(rename = "currencyType")]
    pub currency_type: String,
    pub amount: u32,
    pub enabled: bool,

    /// The name of the wallet.
    pub name: String,
    pub icon: String,

    #[serde(rename = "amountInBaseCurrency")]
    pub amount_in_base_currency: u32,

    #[serde(rename = "isMain")]
    pub is_main: bool,
}

/// The ID of a [`Wallet`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WalletId(String);

impl Display for WalletId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for WalletId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for WalletId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
