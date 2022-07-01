//! Catalog resource definitions
//!
//! See [Scryfall api documentation](https://scryfall.com/docs/api/catalogs)

use reqwest::Method;
use serde::{Deserialize, Serialize};
use url::Url;

use CatalogResource::*;

use crate::resources::{HttpResource, ResourceKind};

/// Endpoints for `/catalog` resource
pub enum CatalogResource {
    /// Binding for endpoint `GET /catalog/ability-words`
    AbilityWords,

    /// Binding for endpoint `GET /catalog/artifact-types`
    ArtifactTypes,

    /// Binding for endpoint `GET /catalog/artist-names`
    ArtistNames,

    /// Binding for endpoint `GET /catalog/card-names`
    CardNames,

    /// Binding for endpoint `GET /catalog/creature-types`
    CreatureTypes,

    /// Binding for endpoint `GET /catalog/enchantment-types`
    EnchantmentTypes,

    /// Binding for endpoint `GET /catalog/keyword-abilities`
    KeywordAbilities,

    /// Binding for endpoint `GET /catalog/keyword-actions`
    KeywordActions,

    /// Binding for endpoint `GET /catalog/land-types`
    LandTypes,

    /// Binding for endpoint `GET /catalog/loyalties`
    Loyalties,

    /// Binding for endpoint `GET /catalog/planeswalker-types`
    PlaneswalkerTypes,

    /// Binding for endpoint `GET /catalog/powers`
    Powers,

    /// Binding for endpoint `GET /catalog/spell-types`
    SpellTypes,

    /// Binding for endpoint `GET /catalog/toughnesses`
    Toughnesses,

    /// Binding for endpoint `GET /catalog/watermarks`
    Watermarks,

    /// Binding for endpoint `GET /catalog/word-bank`
    WordBank,
}

impl HttpResource<Catalog> for CatalogResource {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("catalog/{}", match self {
            AbilityWords => "ability-words",
            ArtifactTypes => "artifact-types",
            ArtistNames => "artist-names",
            CardNames => "card-names",
            CreatureTypes => "creature-types",
            EnchantmentTypes => "enchantment-types",
            KeywordAbilities => "keyword-abilities",
            KeywordActions => "keyword-actions",
            LandTypes => "land-types",
            Loyalties => "loyalties",
            PlaneswalkerTypes => "planeswalker-types",
            Powers => "powers",
            SpellTypes => "spell-types",
            Toughnesses => "toughnesses",
            Watermarks => "watermarks",
            WordBank => "word-bank",
        })
    }
}

/// Basic struct representing a catalog
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Catalog {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub uri: Option<Url>,
    pub total_values: i64,
    pub data: Vec<String>,
}
