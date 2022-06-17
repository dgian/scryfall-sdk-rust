//! Client implementation
//!
//! Async client is the default implementation.
//!
//! Blocking client can be used also by enabling the **blocking** optional feature.

use reqwest::{Client, Error, Method};
use serde::Deserialize;

#[cfg(feature = "blocking")]
pub mod blocking;

/// Scryfall async client
#[derive(Clone)]
pub struct Scryfall<'a> {
    base_url: &'a str,
    http_client: Client
}

impl<'a> Scryfall<'a> {
    /// Creates a client from a URL
    #[must_use]
    pub fn from_url(url: &'a str) -> Self {
        Scryfall {
            base_url: url,
            http_client: Client::builder()
                .user_agent("Rust-SDK: async-clients")
                .build()
                .unwrap_or_default()
        }
    }

    /// Gets a clone of the wrapped Client object
    ///
    /// Cloning the Client happens in order to re-use its connection pool.
    #[must_use]
    pub fn http_client(&self) -> Client {
        self.http_client.clone()
    }

    /// Makes an HTTP request to an endpoint
    pub async fn request<E, R>(&self, endpoint: &E) -> Result<R, Error>
        where E: Endpoint<R>,
              R: for<'de> Deserialize<'de>
    {
        let url = format!("{}/{}", self.base_url, endpoint.path());

        self.http_client()
            .request(endpoint.method(), url)
            .send().await?
            .json().await
    }
}

/// Represents an HTTP endpoint
///
/// This is used as a parameter to [Scryfall](Scryfall)
/// in order to make a request to the api.
pub trait Endpoint<R: for<'de> Deserialize<'de>> {
    /// Defines the HTTP method for the endpoint
    fn method(&self) -> Method;

    /// Defines the path for the endpoint
    ///
    /// The path should be relative to the `base_url` of [Scryfall](Scryfall)
    fn path(&self) -> String;
}
