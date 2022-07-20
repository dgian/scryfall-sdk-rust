//! Card resource definitions
//!
//! See [Scryfall api documentation](https://scryfall.com/docs/api/cards)

use reqwest::Method;
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;
use url::Url;
use strum_macros::Display;
use CardCatalogResource::Autocomplete;
use CardPageResource::Search;
use CardResource::*;
use CardCollectionResource::*;
use crate::HttpResource;
use crate::resources::card_symbols::ColorSymbol;
use crate::resources::catalog::Catalog;
use crate::resources::ResourceKind;

// ---------------------------------------
// --  HTTP resources  -------------------
// ---------------------------------------

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

/// Endpoints for `/cards/*` resource (page)
pub enum CardPageResource {
    /// Binding for endpoint `GET /cards/search`
    ///
    /// Searches for one ore more cards based on specific query parameters.
    ///
    /// The following query parameters are supported:
    ///
    /// | parameter             | usage                      |
    /// |-----------------------|----------------------------|
    /// | q                     | fulltext search query      |
    /// | unique                | omitting similar cards     |
    /// | order                 | field to sort cards by     |
    /// | dir                   | sorting direction          |
    /// | include_extras        | extra cards (e.g. tokens)  |
    /// | include_multilingual  | multilingual card versions |
    /// | include_variations    | rare card variants         |
    /// | page                  | results page number        |
    ///
    /// More info on the above parameters [are provided here](https://scryfall.com/docs/api/cards/search)
    ///
    /// For the `q` parameters, Scryfall provides a very [powerful search syntax](https://scryfall.com/docs/syntax)
    /// which you could use to fine-tune your search queries.
    /// Currently only raw string for `q` is supported. Future versions will provide
    /// a fluent/rust way of defining the query according to the above syntax.
    ///
    /// You can use the `with_q` function to initialise the search params
    /// by only providing the search string and leaving the rest of the query parameters empty.
    ///
    /// # Example
    /// ```
    /// use scryfall_sdk_rust::{CardPageResource, HttpResource};
    /// use scryfall_sdk_rust::resources::cards::SearchQueryParams;
    ///
    /// let search_card = CardPageResource::Search(
    ///     SearchQueryParams::with_q("name")
    /// );
    ///
    /// assert_eq!("cards/search?q=name", search_card.path())
    /// ```
    Search(SearchQueryParams)
}

/// Endpoints for `/cards/*` resource (catalogs)
pub enum CardCatalogResource<'a> {
    /// Binding for endpoint `GET /cards/autocomplete`
    ///
    /// Returns a catalog of cards containing up to 20 card names,
    /// used for autocompletion functionalities.
    ///
    /// See more info for this in the
    /// [official Scryfall documentation](https://scryfall.com/docs/api/cards/autocomplete).
    Autocomplete(&'a str),
}

/// Endpoints for `/cards/collection` resource
pub enum CardCollectionResource {
    /// Binding for endpoint `POST /cards/collection`
    /// 
    /// Accepts a JSON array of card identifiers, 
    /// and returns a List object with the collection of requested cards.
    /// 
    /// Available identifiers are the following:
    /// 
    /// | identifier(s)          | usage                                                  |
    /// |------------------------|--------------------------------------------------------|
    /// | id                     | find a card by Scryfall id                             |
    /// | mtgo_id                | find a card by MTGO id                                 |
    /// | multiverse_id          | find a card by Multiverse id                           |
    /// | oracle_id              | find a card by Oracle id                               |
    /// | illustration_id        | find a card by illustration id (preferred scans)       |
    /// | name                   | find a card by name (newest)                           |
    /// | set + name             | find a card by combination of set and card name        |
    /// | set + collector_number | find a card by combination of set and collector number |
    ///
    /// See more info for this in the 
    /// [official Scryfall documentation](https://scryfall.com/docs/api/cards/collection).
    WithIdentifiers(CardIdentifiers),
}

impl<'a> HttpResource<Card> for CardResource<'a> {
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

impl HttpResource<CardPage> for CardPageResource {
    fn path(&self) -> String {
        format!("cards/search{}", match self {
            Search(params) => params.as_query_str()
        })
    }
}

impl<'a> HttpResource<Catalog> for CardCatalogResource<'a> {
    fn path(&self) -> String {
        format!("cards/{}", match self {
            Autocomplete(q) => format!("autocomplete?q={q}")
        })
    }
}

impl HttpResource<CardCollection> for CardCollectionResource {
    fn method(&self) -> Method {
        match self {
            WithIdentifiers(_) => Method::POST,
        }
    }

    fn path(&self) -> String { 
        match self {
            WithIdentifiers(_) => "cards/collection".into()
        }
    }

    fn json(&self) -> Option<String> {
        match self {
            WithIdentifiers(r) => serde_json::to_string(r).ok(),
        }
    }
}

// ---------------------------------------
// --  Model definitions  ----------------
// ---------------------------------------

/// Basic struct representing a card
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Card {
    pub all_parts: Option<Vec<RelatedCard>>,
    pub arena_id: Option<i32>,
    pub artist: Option<String>,
    pub artist_ids: Vec<Uuid>,
    pub booster: bool,
    pub border_color: String,
    pub card_back_id: Option<Uuid>,
    pub card_faces: Option<Vec<CardFace>>,
    pub cardmarket_id: Option<i32>,
    pub cmc: f64,
    pub collector_number: String,
    pub color_identity: Vec<ColorSymbol>,
    pub color_indicator: Option<Vec<ColorSymbol>>,
    pub colors: Option<Vec<ColorSymbol>>,
    pub content_warning: Option<bool>,
    pub digital: bool,
    pub edhrec_rank: Option<i64>,
    pub finishes: Vec<CardFinish>,
    pub flavor_name: Option<String>,
    pub flavor_text: Option<String>,
    pub foil: bool,
    pub frame: String,
    pub full_art: bool,
    pub games: Vec<GameKind>,
    pub hand_modifier: Option<String>,
    pub highres_image: bool,
    pub id: Uuid,
    pub illustration_id: Option<Uuid>,
    pub image_status: ImageStatus,
    pub image_uris: Option<ImageUris>,
    pub keywords: Vec<String>,
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub lang: String,
    pub layout: Layout,
    pub legalities: Legalities,
    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub mtgo_id: Option<i32>,
    pub mtgo_foil_id: Option<i32>,
    pub multiverse_ids: Option<Vec<i32>>,
    pub name: String,
    pub nonfoil: bool,
    pub oracle_id: Uuid,
    pub oracle_text: Option<String>,
    pub oversized: bool,
    pub penny_rank: Option<i64>,
    pub power: Option<String>,
    pub prices: Prices,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub prints_search_uri: Url,
    pub produced_mana: Option<Vec<ColorSymbol>>,
    pub promo: bool,
    pub promo_types: Option<Vec<String>>,
    pub purchase_uris: Option<PurchaseUris>,
    pub rarity: Rarity,
    pub related_uris: Option<RelatedUris>,
    pub released_at: Date,
    pub reprint: bool,
    pub reserved: bool,
    pub rulings_uri: Url,
    pub scryfall_set_uri: Url,
    pub scryfall_uri: Url,
    pub security_stamp: Option<String>,
    pub set: String,
    pub set_id: String,
    pub set_name: String,
    pub set_search_uri: Url,
    pub set_type: String,
    pub set_uri: Url,
    pub story_spotlight: bool,
    pub tcgplayer_id: Option<i32>,
    pub tcgplayer_etched_id: Option<i32>,
    pub textless: bool,
    pub toughness: Option<String>,
    pub type_line: String,
    pub uri: Url,
    pub variation: bool,
    pub variation_of: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelatedCard {
    pub component: String,
    pub id: Uuid,
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub name: String,
    pub type_line: String,
    pub uri: Url,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CardFinish {
    Etched,
    Foil, 
    Glossy,
    NonFoil, 
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GameKind {
    Arena,
    Mtgo,
    Paper,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImageStatus {
    HighresScan,
    Lowres,
    Missing,
    Placeholder,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    Adventure,
    ArtSeries,
    Augment,
    Class,
    DoubleFacedToken,
    Emblem,
    Flip,
    Host,
    Leveler,
    Meld,
    ModalDfc,
    Normal,
    Planar,
    ReversibleCard,
    Saga,
    Scheme,
    Split,
    Token,
    Transform,
    Vanguard,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Bonus,
    Common,
    Mythic,
    Rare,
    Special,
    Uncommon,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CardPage {
    pub data: Vec<Card>,
    pub has_more: bool,
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub next_page: Option<Url>,
    pub total_cards: i64,
}

/// A struct representing the face of a card
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CardFace {
    pub artist: Option<String>,
    pub artist_id: Option<Uuid>,
    pub cmc: Option<f64>,
    pub color_indicator: Option<Vec<ColorSymbol>>,
    pub colors: Option<Vec<ColorSymbol>>,
    pub flavor_name: Option<String>,
    pub flavor_text: Option<String>,
    pub illustration_id: Option<Uuid>,
    pub image_uris: Option<ImageUris>,
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub layout: Option<Layout>,
    pub loyalty: Option<String>,
    pub mana_cost: String,
    pub name: String,
    pub oracle_id: Option<Uuid>,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub toughness: Option<String>,
    pub type_line: Option<String>,
    pub watermark: Option<String>,
}

/// Container for image URLs
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ImageUris {
    pub art_crop: Url,
    pub border_crop: Url,
    pub large: Url,
    pub normal: Url,
    pub png: Url,
    pub small: Url,
}

/// Container for card legalities
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Legalities {
    pub alchemy: Legality,
    pub brawl: Legality,
    pub commander: Legality,
    pub duel: Legality,
    pub explorer: Legality,
    pub future: Legality,
    pub gladiator: Legality,
    pub historic: Legality,
    pub historicbrawl: Legality,
    pub legacy: Legality,
    pub modern: Legality,
    pub oldschool: Legality,
    pub pauper: Legality,
    pub paupercommander: Legality,
    pub penny: Legality,
    pub pioneer: Legality,
    pub premodern: Legality,
    pub standard: Legality,
    pub vintage: Legality,
}

/// Container for card prices
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Prices {
    pub eur: Option<String>,
    pub eur_foil: Option<String>,
    pub tix: Option<String>,
    pub usd: Option<String>,
    pub usd_etched: Option<String>,
    pub usd_foil: Option<String>,
}

/// Container for card purchase URLs
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PurchaseUris {
    pub cardhoarder: Url,
    pub cardmarket: Url,
    pub tcgplayer: Url,
}

/// Container for other card related URLs
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelatedUris {
    pub edhrec: Option<Url>,
    pub gatherer: Option<Url>,
    pub tcgplayer_infinite_articles: Option<Url>,
    pub tcgplayer_infinite_decks: Option<Url>,
}

/// Card legality enum
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Legality {
    #[serde(rename = "banned")]
    Banned,

    #[serde(rename = "legal")]
    Legal,

    #[serde(rename = "not_legal")]
    NotLegal,

    #[serde(rename = "restricted")]
    Restricted,
}

pub struct SearchQueryParams {
    pub dir: Option<OrderDirection>,
    pub include_extras: Option<bool>,
    pub include_multilingual: Option<bool>,
    pub include_variations: Option<bool>,
    pub order: Option<OrderField>,
    pub page: Option<u32>,
    pub q: String,
    pub unique: Option<UniqueMode>,
}

impl SearchQueryParams {
    pub fn as_query_str(&self) -> String {
        let mut query = format!("?q={}", self.q);

        query.push_str(self.unique.as_ref()
            .map_or("".into(),
                    |mode| format!("&unique={}", mode)
            ).as_str()
        );

        query.push_str(self.order.as_ref()
            .map_or("".into(),
                    |field| format!("&order={}", field)
            ).as_str()
        );

        query.push_str(self.dir.as_ref()
            .map_or("".into(),
                    |dir| format!("&dir={}", dir)
            ).as_str()
        );

        query.push_str(self.include_extras.as_ref()
            .map_or("".into(),
                    |b| format!("&include_extras={}", b)
            ).as_str()
        );

        query.push_str(self.include_multilingual.as_ref()
            .map_or("".into(),
                    |b| format!("&include_multilingual={}", b),
            ).as_str()
        );

        query.push_str(self.include_variations.as_ref()
            .map_or("".into(),
                    |b| format!("&include_variations={}", b),
            ).as_str()
        );

        query.push_str(self.page.as_ref()
            .map_or("".into(),
                    |p| format!("&page={}", p),
            ).as_str()
        );

        query
    }

    pub fn with_q(q: &str) -> Self {
        SearchQueryParams {
            q: q.into(),
            unique: None,
            order: None,
            dir: None,
            include_extras: None,
            include_multilingual: None,
            include_variations: None,
            page: None,
        }
    }
}

#[derive(Default, Display)]
#[strum(serialize_all = "snake_case")]
pub enum UniqueMode {
    Art,
    #[default]
    Cards,
    Prints,
}

#[derive(Default, Display)]
#[strum(serialize_all = "snake_case")]
pub enum OrderField {
    Artist,
    Cmc,
    Color,
    Edhrec,
    Eur,
    #[default]
    Name,
    Penny,
    Power,
    Rarity,
    Released,
    Review,
    Set,
    Tix,
    Toughness,
    Usd,
}

#[derive(Default, Display)]
#[strum(serialize_all = "snake_case")]
pub enum OrderDirection {
    #[default]
    Auto,
    Asc,
    Desc,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CardCollection {
    #[serde(rename = "object")]
    pub kind: ResourceKind,

    pub not_found: Vec<CardIdentifier>,

    #[serde(rename = "data")]
    pub cards: Vec<Card>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct CardIdentifiers {
    pub identifiers: Vec<CardIdentifier>
}

#[derive(Debug, Display, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CardIdentifier {
    IllustrationId { 
        #[serde(rename="illustration_id")] 
        val: String,
    },
    MtgoId { 
        #[serde(rename="mtgo_id")] 
        val: String,
    },
    MutliverseId { 
        #[serde(rename="multiverse_id")] 
        val: u32,
    },
    Name { 
        #[serde(rename="name")] 
        val: String,
    },
    OracleId { 
        #[serde(rename="oracle_id")] 
        val: String,
    },
    ScryfallId { 
        #[serde(rename="id")] 
        val: String,
    },
    SetAndName { 
        set: String,
        name: String, 
    },
    SetAndNumber {
        set: String,

        #[serde(rename="collector_number")] 
        number: String,
    },
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
    fn card_resource_should_return_path_and_method(
        #[case] resource: CardResource,
        #[case] expected: &str
    ) {
        assert_eq!(expected, resource.path());
        assert_eq!(Method::GET, resource.method());
    }

    #[rstest]
    #[case::search(CardPageResource::Search(SearchQueryParams::with_q("test")), "cards/search?q=test")]
    fn card_page_resource_should_return_path_and_method(
        #[case] resource: CardPageResource,
        #[case] expected: &str
     ) {
        assert_eq!(expected, resource.path());
        assert_eq!(Method::GET, resource.method());
    }

    #[rstest]
    #[case::autocomplete(CardCatalogResource::Autocomplete("test"), "cards/autocomplete?q=test")]
    fn card_catalog_resource_should_return_path_and_method(
        #[case] resource: CardCatalogResource,
        #[case] expected: &str
    ) {
        assert_eq!(expected, resource.path());
        assert_eq!(Method::GET, resource.method());
    }

    #[rstest]
    fn card_collection_resource_should_return_path_method_and_json_body() {
        let resource = CardCollectionResource::WithIdentifiers(
            CardIdentifiers {
                identifiers: vec![
                    CardIdentifier::ScryfallId { val: "123".into() }
                ]
            }
        );

        assert_eq!("cards/collection", resource.path());
        assert_eq!(Method::POST, resource.method());
        assert_eq!(String::from("{\"identifiers\":[{\"id\":\"123\"}]}"), resource.json().unwrap());
    }

    #[rstest]
    #[case::all_off("q", None, None, None, None, None, None, None, "?q=q")]
    #[case::all_on("q",
        Some(UniqueMode::default()),
        Some(OrderField::default()),
        Some(OrderDirection::default()),
        Some(true), Some(true), Some(true), Some(1),
        "?q=q&unique=cards&order=name&dir=auto&include_extras=true&include_multilingual=true&include_variations=true&page=1"
    )]
    fn search_query_params_should_parse_as_query_string(
        #[case] q: String,
        #[case] unique: Option<UniqueMode>,
        #[case] order: Option<OrderField>,
        #[case] dir: Option<OrderDirection>,
        #[case] include_extras: Option<bool>,
        #[case] include_multilingual: Option<bool>,
        #[case] include_variations: Option<bool>,
        #[case] page: Option<u32>,
        #[case] expected: String
    ) {
        let params = SearchQueryParams {
            q,
            unique,
            order,
            dir,
            include_extras,
            include_multilingual,
            include_variations,
            page,
        };

        assert_eq!(expected, params.as_query_str())
    }
}
