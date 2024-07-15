use serde::{Deserialize, Serialize};

use crate::wallets::WalletId;

#[derive(Serialize, Deserialize)]
pub struct Exchange {
    /// Source account for the exchange.
    pub source: ExchangeAccount,

    /// Destination account for the exchange.
    pub destination: ExchangeAccount
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeAccount {
    /// Amount for the account.
    pub amount: f64,

    /// Currency used by the account.
    pub currency: WalletId,

    /// Effect of the account balance after the exchange.
    pub balance: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct Balance {

    /// Balance after the exchange.
    pub after: String,

    /// Balance before the exchange.
    pub before: String,
}
