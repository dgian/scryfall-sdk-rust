//! Bulk Data resource definitions
//!
//! See [Scryfall api documentation](https://scryfall.com/docs/api/bulk-data)

use reqwest::Method;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::iso8601;
use url::Url;

use BulkDataResource::*;

use crate::resources::{HttpResource, ResourceKind};

/// Endpoints for `/bulk-data` resource (list)
pub enum BulkDataListResource {
    /// Binding for endpoint `GET /bulk-data`
    All,
}

/// Endpoints for `/bulk-data/*` resource (single)
pub enum BulkDataResource<'a> {
    /// Binding for endpoints:
    /// - `GET /bulk-data/:id`
    /// - `GET /bulk-data/:type`
    ///
    /// The Scryfall api exposes two different endpoints,
    /// but since they provide the same functionality (filter-by-value),
    /// they both are covered by this binding.
    Filter(&'a str),
}

impl HttpResource<BulkDataList> for BulkDataListResource {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("bulk-data")
    }
}

impl<'a> HttpResource<BulkData> for BulkDataResource<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        match self {
            Filter(by) => format!("bulk-data/{}", by),
        }
    }
}

/// Basic struct representing bulk data list
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BulkDataList {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub has_more: bool,
    pub data: Vec<BulkData>,
}

/// A bulk data entry
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BulkData {
    #[serde(rename = "object")]
    pub item_kind: ResourceKind,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: BulkDataKind,
    #[serde(with = "iso8601")]
    pub updated_at: OffsetDateTime,
    pub uri: Url,
    pub name: String,
    pub description: String,
    pub compressed_size: i64,
    pub download_uri: Url,
    pub content_type: String,
    pub content_encoding: String,
}

/// Kind of bulk data
///
/// This refers to Scryfall `bulk_data.type` field
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BulkDataKind {
    /// `type` -> `all_cards`
    #[serde(rename = "all_cards")]
    AllCards,

    /// `type` -> `default_cards`
    #[serde(rename = "default_cards")]
    DefaultCards,

    /// `type` -> `oracle_cards`
    #[serde(rename = "oracle_cards")]
    OracleCards,

    /// `type` -> `rulings`
    #[serde(rename = "rulings")]
    Rulings,

    /// `type` -> `unique_artwork`
    #[serde(rename = "unique_artwork")]
    UniqueArtwork,
}
