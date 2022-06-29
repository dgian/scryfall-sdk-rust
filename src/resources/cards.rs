//! Card resource definitions
//!
//! See [Scryfall api documentation](https://scryfall.com/docs/api/cards)

use reqwest::Method;
use serde::{Deserialize, Serialize};
use time::Date;
use url::Url;
use CardResource::*;
use crate::HttpResource;
use crate::resources::card_symbols::ColorSymbol;
use crate::resources::ResourceKind;

/// Endpoints for `/cards/*` resource (single card)
pub enum CardResource<'a> {
    /// Binding for endpoint `GET /cards/:id`
    ///
    /// Get a single card by its Scryfall id
    ById(&'a str),

    /// Binding for endpoint `GET /cards/arena/:id`
    ///
    /// Get a single card by its Arena id
    ByArenaId(&'a str),

    /// Binding for endpoint `GET /cards/cardmarket/:id`
    ///
    /// Get a single card by its Cardmarket id
    ByCardmarketId(&'a str),

    /// Binding for endpoint `GET /cards/:code/:number`
    ///
    /// Get a single card by its code and collector number
    ByCode(&'a str, &'a str),

    /// Binding for endpoint `GET /cards/mtgo/:id`
    ///
    /// Get a single card by its MTGO id
    ByMtgoId(&'a str),

    /// Binding for endpoint `GET /cards/multiverse/:id`
    ///
    /// Get a single card by its Multiverse id
    ByMultiverseId(&'a str),

    /// Binding for endpoint `GET /cards/tcgplayer/:id`
    ///
    /// Get a single card by its Tcgplayer id
    ByTcgplayerId(&'a str),

    /// Binding for endpoint `GET /cards/named?exact={name}`
    ///
    /// Get a single card by its exact name
    NamedExact(&'a str),

    /// Binding for endpoint `GET /cards/named?fuzzy={name}`
    ///
    /// Get a single card by fuzzy searching its name.
    /// If exact match is found it is returned instead
    NamedFuzzy(&'a str),

    /// Binding for endpoint `GET /cards/random`
    ///
    /// Get a single card at random.
    /// An optional `q` query parameter can provided
    /// in order to limit the pool of cards.
    Random(Option<&'a str>),
}

impl<'a> HttpResource<Card> for CardResource<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("cards/{}", match self {
            ById(id) => id.to_string(),

            ByArenaId(id) => format!(
                "arena/{id}"
            ),
            ByCardmarketId(id) => format!(
                "cardmarket/{id}"
            ),
            ByCode(code, number) => format!(
                "{code}/{number}"
            ),
            ByMtgoId(id) => format!(
                "mtgo/{id}"
            ),
            ByMultiverseId(id) => format!(
                "multiverse/{id}"
            ),
            ByTcgplayerId(id) => format!(
                "tcgplayer/{id}"
            ),
            NamedExact(name) => format!(
                "named?exact={name}"
            ),
            NamedFuzzy(name) => format!(
                "named?fuzzy={name}"
            ),
            Random(query) => format!(
                "random{}", query
                    .map(|q| format!("?q={q}"))
                    .unwrap_or("".into())
            ),
        })
    }
}

/// Basic struct representing a card
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Card {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub id: String,                     // TODO: uuid
    pub oracle_id: String,              // TODO: uuid
    pub multiverse_ids: Vec<i64>,
    pub tcgplayer_id: i64,
    pub name: String,
    pub lang: String,
    pub released_at: Date,
    pub uri: Url,
    pub scryfall_uri: Url,
    pub layout: String,                 // TODO: enum??
    pub highres_image: bool,
    pub image_status: String,           // TODO: enum??
    pub image_uris: ImageUris,
    pub mana_cost: String,
    pub cmc: f64,
    pub type_line: String,
    pub colors: Vec<ColorSymbol>,
    pub color_identity: Vec<ColorSymbol>,
    pub keywords: Vec<String>,
    pub card_faces: Vec<CardFace>,
    pub legalities: Legalities,
    pub games: Vec<String>,             // TODO: enum??
    pub reserved: bool,
    pub foil: bool,
    pub nonfoil: bool,
    pub finishes: Vec<String>,          // TODO: enum??
    pub oversized: bool,
    pub promo: bool,
    pub reprint: bool,
    pub variation: bool,
    pub set_id: String,
    pub set: String,
    pub set_name: String,
    pub set_type: String,
    pub set_uri: Url,
    pub set_search_uri: Url,
    pub scryfall_set_uri: Url,
    pub rulings_uri: Url,
    pub prints_search_uri: Url,
    pub collector_number: String,
    pub digital: bool,
    pub rarity: String,                 // TODO: enum
    pub card_back_id: String,           // TODO: uuid
    pub artist: String,
    pub artist_ids: Vec<String>,        // TODO: uuid
    pub illustration_id: String,        // TODO: uuid
    pub border_color: String,
    pub frame: String,
    pub security_stamp: String,
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    pub story_spotlight: bool,
    pub edhrec_rank: i64,
    pub penny_rank: i64,
    pub prices: Prices,
    pub related_uris: RelatedUris,
    pub purchase_uris: PurchaseUris,
}

/// A struct representing the face of a card
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CardFace {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub name: String,
    pub mana_cost: String,
    pub type_line: String,
    pub oracle_text: String,
    pub artist: Option<String>,
    pub artist_id: Option<String>,      // TODO: uuid
    pub illustration_id: Option<String>,// TODO: uuid
    pub flavor_name: Option<String>,
}

/// Container for image URLs
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ImageUris {
    pub small: Url,
    pub normal: Url,
    pub large: Url,
    pub png: Url,
    pub art_crop: Url,
    pub border_crop: Url,
}

/// Container for card legalities
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Legalities {
    pub standard: Legality,
    pub future: Legality,
    pub historic: Legality,
    pub gladiator: Legality,
    pub pioneer: Legality,
    pub explorer: Legality,
    pub modern: Legality,
    pub legacy: Legality,
    pub pauper: Legality,
    pub vintage: Legality,
    pub penny: Legality,
    pub commander: Legality,
    pub brawl: Legality,
    pub historicbrawl: Legality,
    pub alchemy: Legality,
    pub paupercommander: Legality,
    pub duel: Legality,
    pub oldschool: Legality,
    pub premodern: Legality,
}

/// Container for card prices
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Prices {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub usd_etched: Option<String>,
    pub eur: Option<String>,
    pub eur_foil: Option<String>,
    pub tix: Option<String>,
}

/// Container for card purchase URLs
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PurchaseUris {
    pub tcgplayer: Url,
    pub cardmarket: Url,
    pub cardhoarder: Url,
}

/// Container for other card related URLs
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelatedUris {
    pub gatherer: Url,
    pub tcgplayer_infinite_articles: Url,
    pub tcgplayer_infinite_decks: Url,
    pub edhrec: Url,
}

/// Card legality enum
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Legality {
    #[serde(rename = "legal")]
    Legal,

    #[serde(rename = "not_legal")]
    NotLegal
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::by_id(CardResource::ById("123"), "cards/123")]
    #[case::by_arena_id(CardResource::ByArenaId("123"), "cards/arena/123")]
    #[case::by_cardmarket_id(CardResource::ByCardmarketId("123"), "cards/cardmarket/123")]
    #[case::by_code(CardResource::ByCode("123", "456"), "cards/123/456")]
    #[case::by_mtgo_id(CardResource::ByMtgoId("123"), "cards/mtgo/123")]
    #[case::by_multiverse_id(CardResource::ByMultiverseId("123"), "cards/multiverse/123")]
    #[case::by_tcgplayer_id(CardResource::ByTcgplayerId("123"), "cards/tcgplayer/123")]
    #[case::named_exact(CardResource::NamedExact("name"), "cards/named?exact=name")]
    #[case::named_fuzzy(CardResource::NamedFuzzy("name"), "cards/named?fuzzy=name")]
    #[case::random(CardResource::Random(None), "cards/random")]
    #[case::random(CardResource::Random(Some("name")), "cards/random?q=name")]
    fn card_set_resource_should_return_path_and_method(
        #[case] resource: CardResource,
        #[case] expected: &str
    ) {
        assert_eq!(expected, resource.path());
        assert_eq!(Method::GET, resource.method());
    }
}
