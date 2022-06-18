//! Blocking client implementation
//!
//! Blocking has to be enabled by the **blocking** optional feature.

use reqwest::blocking::Client;
use reqwest::Error;
use serde::Deserialize;

use crate::HttpResource;

/// Scryfall blocking client
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
                .user_agent("Rust-SDK: sync-client")
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
    pub fn request<E, R>(&self, endpoint: &E) -> Result<R, Error>
        where E: HttpResource<R>,
              R: for<'de> Deserialize<'de>
    {
        let url = format!("{}/{}", self.base_url, endpoint.path());

        self.http_client()
            .request(endpoint.method(), url)
            .send()?
            .json()
    }
}
