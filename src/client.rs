//! Client implementation
//!
//! Async client is the default implementation.
//!
//! Blocking client can be used also by enabling the **blocking** optional feature.

use reqwest::{Client, Error};
use serde::Deserialize;
use crate::HttpResource;

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
    pub async fn request<R, O>(&self, resource: &R) -> Result<O, Error>
        where R: HttpResource<O>,
              O: for<'de> Deserialize<'de>
    {
        let url = format!("{}/{}", self.base_url, resource.path());

        self.http_client()
            .request(resource.method(), url)
            .send().await?
            .json().await
    }
}

impl<'a> Default for Scryfall<'a> {
    fn default() -> Self {
        Scryfall::from_url("https://api.scryfall.com")
    }
}