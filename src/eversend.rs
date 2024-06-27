use crate::{auth::Auth, core::{ApiToken, ClientError, ClientId, EversendError, BASE_URL}, wallets::Wallets};

/// The Eversend client.
pub struct Eversend {
    api_token: Option<ApiToken>,
    base_url: String,
    client: reqwest::Client,
    client_id: ClientId,
    client_secret: String, // TODO: Create custom type for this
}

impl Eversend {
    /// Returns a new instance of the Eversend client using the provided API client ID, and Secret.
    pub fn new(client_id: &ClientId, client_secret: &str) -> Self {
        EversendBuilder::new(
            client_id,
            client_secret
        ).build()
    }

    /// Returns a [`EversendBuilder`] that may be used to construct an Eversend client.
    pub fn builder<'a>(client_id: &'a ClientId, client_secret: &'a str) -> EversendBuilder<'a> {
        EversendBuilder::new(client_id, client_secret)
    }

    pub fn base_url(&self) -> &str {
        &self.base_url.as_str()
    }

    pub fn client_secret(&self) -> &str {
        &self.client_secret.as_str()
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    pub(crate) fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn api_token(&self) -> Result<&ApiToken, ClientError> {
        if let Some(token) = &self.api_token {
            return Ok(token);
        }

        Err(ClientError::new(
            EversendError::ApiTokenMissing,
            "API token is not set".to_owned(),
        ))
    }

    /// Returns an [`Auth`] instance.
    pub fn auth(&self) -> Auth {
        Auth::new(self)
    }

    /// Returns an [`Wallets`] instance.
    pub fn wallets(&self) -> Wallets {
        Wallets::new(self)
    }
}

/// A builder for an Eversend client.
pub struct EversendBuilder<'a> {
    api_token: Option<ApiToken>,
    base_url: String,
    client_id: &'a ClientId,
    client_secret: &'a str,
}

impl<'a> EversendBuilder<'a> {
    /// Returns a new [`EversendBuilder`] using the provided API client ID, and Secret.
    pub fn new(client_id: &'a ClientId, client_secret: &'a str) -> Self {
        Self {
            api_token: None,
            base_url: BASE_URL.to_string(),
            client_id,
            client_secret,
        }
    }

    /// Consumes the builder and returns the constructed Eversend client.
    pub fn build(self) -> Eversend {
        let client = reqwest::Client::builder()
            // .user_agent(concat!("eversend-rust/", env!("CARGO_PKG_VERSION")))
            .build()
            .unwrap();

        Eversend {
            api_token: None,
            base_url: self.base_url,
            client_secret: self.client_secret.to_owned(),
            client_id: self.client_id.to_owned(),
            client,
        }
    }

    /// Sets the base URL of the Eversend API that the client should point to.
    pub fn set_base_url(mut self, base_url: &'a str) -> EversendBuilder {
        self.base_url = base_url.to_string();
        self
    }

    /// Sets the client secret of the Eversend API that the client should point to.
    pub fn set_client_secret(mut self, client_secret: &'a str) -> EversendBuilder {
        self.client_secret = client_secret;
        self
    }

    /// Sets the client ID of the Eversend API that the client should point to.
    pub fn set_client_id(mut self, client_id: &'a ClientId) -> EversendBuilder {
        self.client_id = client_id;
        self
    }

    /// Sets the base URL of the Eversend API that the client should point to.
    pub fn set_api_token(mut self, api_token: &'a ApiToken) -> EversendBuilder {
        self.api_token = Some(api_token.to_owned());
        self
    }
}

#[cfg(test)]
mod test {
    // use mockito::mock;
    use super::*;

    #[test]
    fn it_supports_setting_the_base_url_through_the_builder() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456781")
        )
            .set_base_url("https://auth.your-app.com")
            .build();

        assert_eq!(
            eversend.base_url(),
            "https://auth.your-app.com"
        );
    }

    #[test]
    fn it_supports_setting_the_secret_through_the_builder() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_some_client_id"),
            &String::from("sk_some_client_secret")
        )
            .set_client_secret(&String::from("sk_another_client_secret"))
            .build();

        assert_eq!(eversend.client_secret(), &String::from("sk_another_client_secret"))
    }

    #[test]
    fn it_supports_setting_the_client_id_through_the_builder() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_some_client_id"),
            &String::from("sk_some_client_secret")
        )
            .set_client_id(&ClientId::from("sk_another_client_id"))
            .build();

        assert_eq!(eversend.client_id(), &ClientId::from("sk_another_client_id"))
    }
}
