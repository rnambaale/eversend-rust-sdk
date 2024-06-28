use async_trait::async_trait;
use crate::{wallets::{types::Wallet, Wallets}, ApiResponseList};

/// [Eversend Docs: List Wallets](https://eversend.readme.io/reference/get-wallets)
#[async_trait]
pub trait GetWallets {
    /// Retrieves a list of [`Wallets`]s.
    ///
    /// [Eversend Docs: List Wallets](https://eversend.readme.io/reference/get-wallets)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::wallets::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let wallets = eversend
    ///         .wallets()
    ///         .get_wallets()
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn get_wallets(
        &self,
    ) -> Result<ApiResponseList<Wallet>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> GetWallets for Wallets<'a> {
    async fn get_wallets(
        &self,
    ) -> Result<ApiResponseList<Wallet>, Box<dyn std::error::Error>> {
        let url = format!("{}/wallets", self.eversend.base_url());
        let wallets = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            // .handle_unauthorized_or_generic_error()?
            .json::<ApiResponseList<Wallet>>()
            .await?;

        Ok(wallets)
    }
}
