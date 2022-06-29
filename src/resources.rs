//! Scryfall API resources (root module)

use reqwest::Method;
use serde::{Deserialize, Serialize};

pub mod bulk_data;
pub mod catalog;
pub mod card_symbols;
pub mod card_sets;
pub mod cards;
pub mod rulings;

/// Represents an HTTP resource (endpoint)
///
/// This is used as a parameter to [Scryfall](super::Scryfall)
/// in order to make a request to the api.
pub trait HttpResource<R: for<'de> Deserialize<'de>> {
    /// Defines the HTTP method for the endpoint
    fn method(&self) -> Method;

    /// Defines the path for the endpoint
    ///
    /// The path should be relative to the `base_url` of [Scryfall](super::Scryfall)
    fn path(&self) -> String;

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
