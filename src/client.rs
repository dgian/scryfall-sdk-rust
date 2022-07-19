//! Client implementation
//!
//! Async client is the default implementation.
//!
//! Blocking client can be used also by enabling the **blocking** optional feature.

use reqwest::Error;
use reqwest::RequestBuilder;
use reqwest::Client;
use serde::Deserialize;
use crate::HttpResource;
use crate::resources::Response;
use crate::resources::errors::ErrorBody;

#[cfg(feature = "blocking")]
pub mod blocking;

type ResponseResult<M> = Result<Response<M>, Error>;
type BodyResult<M> = Result<M, ErrorBody>;

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
    pub async fn request<R, M>(&self, resource: &R) -> BodyResult<M>
        where R: HttpResource<M>,
              M: for<'de> Deserialize<'de>
    {
        let req = self.build_request(resource);

        match req.send().await {
            Ok(req_ok) => {
                let res = req_ok.json::<Response<M>>().await;
                self.extract_body(res)
            },
            Err(e) => Result::Err(ErrorBody::from_reqwest_error(e)),
        }
    }

    fn build_request<R, M>(&self, resource: &R) -> RequestBuilder
        where R: HttpResource<M>,
              M: for<'de> Deserialize<'de>
    {
        let url = format!("{}/{}", self.base_url, resource.path());

        let mut req = self.http_client()
            .request(resource.method(), url)
            .header("Content-Type", "application/json");
       
        if let Some(b) = resource.json() {
            req = req.body(b);
        }

        req
    }

    fn extract_body<M>(&self, result: ResponseResult<M>) -> BodyResult<M> 
        where M: for<'de> Deserialize<'de>
    {
        match result {
            Ok(response) => match response {
                Response::Ok(body_ok) => Result::Ok(body_ok),
                Response::Err(body_err) => Result::Err(body_err),
            }    
            Err(e) => Result::Err(ErrorBody::from_reqwest_error(e)),
        }
    }
}

impl<'a> Default for Scryfall<'a> {
    fn default() -> Self {
        Scryfall::from_url("https://api.scryfall.com")
    }
}