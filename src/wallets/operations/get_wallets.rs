use async_trait::async_trait;
use crate::wallets::{types::Wallet, Wallets};


/// [Eversend Docs: List Wallets](https://eversend.readme.io/reference/get-wallets)
#[async_trait]
pub trait GetWallets {
    /// Retrieves a list of [`Wallets`]s.
    ///
    /// [Eversend Docs: List Wallets](https://eversend.readme.io/reference/get-wallets)
    ///
    async fn get_wallets(
        &self,
    ) -> Result<Vec<Wallet>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> GetWallets for Wallets<'a> {
    async fn get_wallets(
        &self,
    ) -> Result<Vec<Wallet>, Box<dyn std::error::Error>> {
        let url = format!("{}/wallets", self.eversend.base_url());
        let wallets = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            // .handle_unauthorized_or_generic_error()?
            .json::<Vec<Wallet>>()
            .await?
            ;

        Ok(wallets)
    }
}
