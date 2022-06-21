//! Card sets resource definitions
//!
//! See [Scryfall api documentation](https://scryfall.com/docs/api/sets)

use reqwest::Method;
use serde::{Deserialize, Serialize};
use time::Date;
use url::Url;
use CardSetResource::{Filter, WithTcgPlayerId};

use crate::HttpResource;
use crate::resources::ResourceKind;

/// Endpoints for `/sets` resource (list)
pub enum CardSetListResource {
    /// Binding for endpoint `GET /sets`
    All,
}

/// Endpoints for `/sets/*` resource (single)
pub enum CardSetResource<'a> {
    /// Binding for endpoints:
    /// - `GET /sets/:code`
    /// - `GET /sets/:id`
    ///
    /// The Scryfall api exposes two different endpoints,
    /// but since they provide the same functionality (filter-by-value),
    /// they both are covered by this binding.
    Filter(&'a str),

    /// Binding for endpoint `GET /sets/tcgplayer/:id`
    WithTcgPlayerId(&'a str),
}

impl HttpResource<CardSetList> for CardSetListResource {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        match self {
            _ => format!("sets")
        }
    }
}

impl<'a> HttpResource<CardSet> for CardSetResource<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        let path = "sets";

        match self {
            Filter(by) => format!("{}/{}", path, by),
            WithTcgPlayerId(id) => format!("{}/tcgplayer/{}", path, id),
        }
    }
}

/// Basic struct representing card set list
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CardSetList {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub has_more: bool,
    pub data: Vec<CardSet>,
}

/// A card set
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CardSet {
    #[serde(rename = "object")]
    pub item_kind: ResourceKind,
    pub id: String,
    pub code: String,
    pub mtgo_code: Option<String>,
    pub arena_code: Option<String>,
    pub name: String,
    pub uri: Url,
    pub scryfall_uri: Url,
    pub search_uri: Url,
    pub released_at: Date,
    #[serde(rename = "set_type")]
    pub kind: SetKind,
    pub card_count: i64,
    pub digital: bool,
    pub nonfoil_only: bool,
    pub foil_only: bool,
    pub icon_svg_uri: Url,
    pub tcgplayer_id: Option<i64>,
    pub parent_set_code: Option<String>,
    pub block_code: Option<String>,
    pub block: Option<String>,
}

/// Kind of card set
///
/// This refers to Scryfall `set.set_type` field
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SetKind {

    /// `set_type` -> `alchemy`
    #[serde(rename = "alchemy")]
    Alchemy,

    /// `set_type` -> `archenemy`
    #[serde(rename = "archenemy")]
    Archenemy,

    /// `set_type` -> `arsenal`
    #[serde(rename = "arsenal")]
    Arsenal,

    /// `set_type` -> `box`
    #[serde(rename = "box")]
    Box,

    /// `set_type` -> `commander`
    #[serde(rename = "commander")]
    Commander,

    /// `set_type` -> `core`
    #[serde(rename = "core")]
    Core,

    /// `set_type` -> `draft_innovation`
    #[serde(rename = "draft_innovation")]
    DraftInnovation,

    /// `set_type` -> `duel_deck`
    #[serde(rename = "duel_deck")]
    DuelDeck,

    /// `set_type` -> `expansion`
    #[serde(rename = "expansion")]
    Expansion,

    /// `set_type` -> `from_the_vault`
    #[serde(rename = "from_the_vault")]
    FromTheVault,

    /// `set_type` -> `funny`
    #[serde(rename = "funny")]
    Funny,

    /// `set_type` -> `masterpiece`
    #[serde(rename = "masterpiece")]
    Masterpiece,

    /// `set_type` -> `masters`
    #[serde(rename = "masters")]
    Masters,

    /// `set_type` -> `memorabilia`
    #[serde(rename = "memorabilia")]
    Memorabilia,

    /// `set_type` -> `planechase`
    #[serde(rename = "planechase")]
    Planechase,

    /// `set_type` -> `premium_deck`
    #[serde(rename = "premium_deck")]
    PremiumDeck,

    /// `set_type` -> `promo`
    #[serde(rename = "promo")]
    Promo,

    /// `set_type` -> `spellbook`
    #[serde(rename = "spellbook")]
    Spellbook,

    /// `set_type` -> `starter`
    #[serde(rename = "starter")]
    Starter,

    /// `set_type` -> `token`
    #[serde(rename = "token")]
    Token,

    /// `set_type` -> `treasure_chest`
    #[serde(rename = "treasure_chest")]
    TreasureChest,

    /// `set_type` -> `vanguard`
    #[serde(rename = "vanguard")]
    Vanguard,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn card_set_list_resource_should_return_path_and_method() {
        let resource = CardSetListResource::All;

        assert_eq!("sets", resource.path());
        assert_eq!(Method::GET, resource.method());
    }

    #[rstest]
    #[case::filter(CardSetResource::Filter("id"), "sets/id")]
    #[case::with_tcgplayerid(CardSetResource::WithTcgPlayerId("id"), "sets/tcgplayer/id")]
    fn card_set_resource_should_return_path_and_method(
        #[case] resource: CardSetResource,
        #[case] expected: &str
    ) {
        assert_eq!(expected, resource.path());
        assert_eq!(Method::GET, resource.method());
    }
}
