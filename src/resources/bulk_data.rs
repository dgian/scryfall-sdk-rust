//! Bulk Data resource definitions

use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use url::Url;

use BulkDataResource::*;

use crate::resources::{HttpResource, ResourceKind};

/// Endpoints for [Bulk Data resource](https://scryfall.com/docs/api/bulk-data)
pub enum BulkDataResource<'a> {
    /// Get all bulk data. [API reference](https://scryfall.com/docs/api/bulk-data/all)
    All,

    /// Get bulk data filtered by.
    ///
    /// Original Scryfall API exposes two endpoints:
    /// - filter-by-id ([API reference](https://scryfall.com/docs/api/bulk-data/id))
    /// - filter-by-type ([API reference](https://scryfall.com/docs/api/bulk-data/type))
    ///
    /// Here, there is no distinction between them and they both are covered by this resource.
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
