//! Scryfall API resources (root module)

use crate::resources::errors::ErrorBody;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub mod bulk_data;
pub mod card_sets;
pub mod card_symbols;
pub mod cards;
pub mod catalog;
pub mod errors;
pub mod rulings;

/// Represents an HTTP resource (endpoint)
///
/// This is used as a parameter to [Scryfall](super::Scryfall)
/// in order to make a request to the api.
pub trait HttpResource<R: for<'de> Deserialize<'de>> {
    /// Defines the HTTP method for the endpoint
    fn method(&self) -> Method {
        Method::GET
    }

    /// Defines the path for the endpoint
    ///
    /// The path should be relative to the `base_url` of [Scryfall](super::Scryfall)
    fn path(&self) -> String;

    /// Defines the (optional) json body when requesting the endpoint.
    /// 
    /// This is useful in cases of POST/PUT/PATCH etc.
    /// By default it is `None`.
    fn json(&self) -> Option<String> {
        None
    }

    /// Strips the query parameters (if any) from the endpoint path
    /// 
    /// # Example
    /// ```
    /// use scryfall_sdk_rust::HttpResource;
    /// 
    /// struct SomeResource;
    /// impl HttpResource<String> for SomeResource {
    ///     fn path(&self) -> String {
    ///         "somepath?aQueryParam=123".into()
    ///     }
    /// }
    ///
    /// let res = SomeResource;
    /// assert_eq!("somepath", res.path_without_query())
    /// ```
    fn path_without_query(&self) -> String {
        self.path()
            .chars()
            .take(self
                .path()
                .find("?")
                .unwrap_or(self.path().len())
            )
            .collect()
    }
}

/// Represents a Response with two different states:
/// 
/// - Ok -> containing a Model representation for the resource (e.g. Card)
/// - Err -> containing an error response with a specified ErrorBody (e.g. 404 errors)
/// 
/// The distinction is done based on the `object` field in the response
/// which is de-serialized in ResourceKind::Error when it is `error` and to everything
/// else when there is real response with the respective object.__rust_force_expr!
/// 
/// You can see more info on error response model in [Scryfall offical API documenation.](https://scryfall.com/docs/api/errors)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Response<M>{
    Ok(M),
    Err(ErrorBody),
}

/// Kind of resource
///
/// Scryfall API uses `object` field on each resource to denote its type.
///
/// Essentially this includes all the available core resources from the API
/// plus `list` which refers to a collection of resources.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ResourceKind {
    /// `object` -> `bulk_data`
    #[serde(rename = "bulk_data")]
    BulkData,

    /// `object` -> `card`
    #[serde(rename = "card")]
    Card,

    /// `object` -> `card_face`
    #[serde(rename = "card_face")]
    CardFace,

    /// `object` -> `card_symbol`
    #[serde(rename = "card_symbol")]
    CardSymbol,

    /// `object` -> `catalog`
    #[serde(rename = "catalog")]
    Catalog,

    /// `object` -> `error`
    #[serde(rename = "error")]
    Error,

    /// `object` -> `list`
    #[serde(rename = "list")]
    List,

    /// `object` -> `mana_cost`
    #[serde(rename = "mana_cost")]
    ManaCost,

    /// `object` -> `ruling`
    #[serde(rename = "ruling")]
    Ruling,

    /// `object` -> `set`
    #[serde(rename = "set")]
    Set,
}
