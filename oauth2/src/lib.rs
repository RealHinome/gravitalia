//! Safe and tiny implementation of OAuth2 in Rust.
//! library supports PKCE and ``
//!
//! # Example
//! ```rust
//! let config = Configuration::default()
//!     .client_id("my_cool_client_id")
//!     .secret("secret_key")
//!     .redirect_url("https://api.gravitalia.com/callback")
//!     .server_url("https://oauth.gravitalia.com/")
//!     .add_scope("identity")
//!     .build()?;
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![deny(dead_code, unused_imports, unused_mut, missing_docs)]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::Deserialize;

/// Configuration requirements for managing an OAuth 2.0 client.
#[derive(Default, Debug, Clone)]
pub struct Configuration {
    /// The application's public identifier.
    client_id: &'static str,
    /// The application's secret key to get access token.
    client_secret: &'static str,
    /// The URL where the client will receive callbacks after authorization.
    redirect_uri: &'static str,
    /// OAuth 2.0 authorization server (AS) URL.
    server_url: &'static str,
    /// Protect against CSRF attack.
    state: String,
    /// Wether use PKCE security.
    /// PKCE will be preferred to state if both are activated.
    pkce: bool,
    /// List of application scopes.
    scopes: Vec<&'static str>,
}

impl Configuration {
    /// Modify `client_id` in configuration.
    pub fn client_id(mut self, client_id: &'static str) -> Self {
        self.client_id = client_id;
        self
    }

    /// Modify `client_secret` in configuration.
    pub fn secret(mut self, client_secret: &'static str) -> Self {
        self.client_secret = client_secret;
        self
    }

    /// Modify `redirect_uri` in configuration.
    pub fn redirect_url(mut self, url: &'static str) -> Self {
        self.redirect_uri = url;
        self
    }

    /// Modify `server_url` in configuration.
    /// Make sur there are `/` at the end.
    pub fn server_url(mut self, url: &'static str) -> Self {
        self.server_url = url;
        self
    }

    /// Set a `state`.
    pub fn state<T>(mut self, state: T) -> Self
    where
        T: ToString,
    {
        self.state = state.to_string();
        self
    }

    /// Set a crypto-secure random `state`.
    pub fn random_state(mut self) -> Self {
        self.state = crypto::random_string(19);
        self
    }

    /// Set `pkce` to true.
    pub fn use_pkce(mut self) -> Self {
        self.pkce = true;
        self
    }

    /// Add new scope to `scopes`.
    pub fn add_scope(mut self, scope: &'static str) -> Self {
        self.scopes.push(scope);
        self
    }

    /// Set scopes to `scopes`.
    pub fn scopes(mut self, scopes: Vec<&'static str>) -> Self {
        self.scopes = scopes;
        self
    }

    /// Finishes configuration and enables full use of OAuth 2.0 client.
    pub fn build(self) -> Result<Client, String> {
        let code_verifier = if self.pkce {
            // Use maximum characters value for PCKE.
            Some(crypto::random_string(128))
        } else {
            None
        };

        Ok(Client {
            configuration: self,
            code_verifier: code_verifier.clone(),
            code_challenge: code_verifier.map(|key| {
                crypto::hash::sha256(key.as_bytes()).unwrap_or_default()
            }),
        })
    }
}

/// OAuth 2.0 final client.
#[derive(Default, Debug, Clone)]
pub struct Client {
    /// Application configuration.
    configuration: Configuration,
    /// PCKE plaintext key.
    code_verifier: Option<String>,
    /// PCKE encrypted key with SHA256.
    code_challenge: Option<String>,
}

impl Client {
    /// Create redirection URL associated with the authorization server with
    /// every needed informations on query string.
    pub fn redirect_url(&self) -> String {
        let mut url = self.configuration.server_url.to_string()
            + "oauth/authorize?response_type=code&client_id="
            + self.configuration.client_id
            + "&scope="
            + self.configuration.scopes.join("%20").as_str()
            + "&redirect_uri="
            + self.configuration.redirect_uri;

        if let Some(code_challenge) = self.code_challenge.clone() {
            url = url
                + "&code_challenge_method=S256&code_challenge="
                + &code_challenge;
        } else if !self.configuration.state.is_empty() {
            url = url + "&state=" + &self.configuration.state;
        }

        url
    }

    /// Obtain an access token from authorization_code.
    pub fn access_token(
        &self,
        authorization_code: &'static str,
    ) -> Result<AccessToken, ureq::Error> {
        let body = if self.configuration.pkce {
            ureq::json!({
                "grant_type": "authorization_code",
                "code": authorization_code,
                "redirect_uri": self.configuration.redirect_uri,
                "code_verifier": self.code_verifier,
            })
        } else {
            ureq::json!({
                "grant_type": "authorization_code",
                "code": authorization_code,
                "redirect_uri": self.configuration.redirect_uri,
            })
        };

        Ok(ureq::post(
            (self.configuration.server_url.to_string() + "oauth/token")
                .as_str(),
        )
        .set("Content-Type", "application/x-www-form-urlencoded")
        .set(
            "Authorization",
            ("Basic ".to_string()
                + &URL_SAFE.encode(
                    self.configuration.client_id.to_string()
                        + ":"
                        + self.configuration.client_secret,
                ))
                .as_str(),
        )
        .send_json(body)
        .unwrap()
        .into_json::<AccessToken>()?)
    }
}

/// JSON response from OAuth server to get access token.
#[derive(Debug, Deserialize)]
pub struct AccessToken {
    /// Token to access granted data.
    pub access_token: String,
    /// access_token validity period.
    pub expires_in: u64,
    /// Token to recreate an acces_token.
    pub refresh_token: Option<String>,
    /// refresh_token validity period.
    pub refresh_token_expires_in: Option<u64>,
    /// List of accesses granted.
    pub scope: String,
}
