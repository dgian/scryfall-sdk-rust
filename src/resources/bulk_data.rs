//! Bulk Data resource definitions
//!
//! See [Scryfall api documentation](https://scryfall.com/docs/api/bulk-data)

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use url::Url;

use BulkDataResource::*;

use crate::resources::{HttpResource, ResourceKind};

/// Endpoints for `/bulk-data` resource
pub enum BulkDataResource<'a> {
    /// Binding for endpoint `GET /bulk-data`
    All,

    /// Binding for endpoints:
    /// - `GET /bulk-data/:id`
    /// - `GET /bulk-data/:type`
    ///
    /// The Scryfall api exposes two different endpoints,
    /// but since they provide the same functionality (filter-by-value),
    /// they both are covered by this binding.
    Filter(&'a str),
}

impl<'a> HttpResource<BulkData> for BulkDataResource<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        let path = "bulk-data";

        match self {
            All => path.into(),
            Filter(f) => format!("{}/{}", path, f),
        }
    }
}

/// Basic struct representing bulk data container
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BulkData {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub has_more: bool,
    pub data: Vec<BulkDataEntry>,
}

/// A bulk data entry
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BulkDataEntry {
    #[serde(rename = "object")]
    pub item_kind: ResourceKind,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: EntryKind,
    pub updated_at: DateTime<Utc>,
    pub uri: Url,
    pub name: String,
    pub description: String,
    pub compressed_size: i64,
    pub download_uri: Url,
    pub content_type: String,
    pub content_encoding: String,
}

/// Kind of bulk data entry
///
/// This refers to Scryfall `bulk_data.type` field
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum EntryKind {
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
