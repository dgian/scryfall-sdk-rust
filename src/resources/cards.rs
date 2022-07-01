//! Card resource definitions
//!
//! See [Scryfall api documentation](https://scryfall.com/docs/api/cards)

use reqwest::Method;
use serde::{Deserialize, Serialize};
use time::Date;
use url::Url;
use strum_macros::Display;
use CardPageResource::Search;
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

impl HttpResource<CardPage> for CardPageResource {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("cards/search{}", match self {
            Search(params) => params.as_query_str()
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
    pub tcgplayer_id: Option<i64>,
    pub name: String,
    pub lang: String,
    pub released_at: Date,
    pub uri: Url,
    pub scryfall_uri: Url,
    pub layout: String,                         // TODO: enum??
    pub highres_image: bool,
    pub image_status: String,                   // TODO: enum??
    pub image_uris: Option<ImageUris>,
    pub mana_cost: Option<String>,
    pub cmc: f64,
    pub type_line: String,
    pub colors: Option<Vec<ColorSymbol>>,
    pub color_identity: Vec<ColorSymbol>,
    pub keywords: Vec<String>,
    pub card_faces: Option<Vec<CardFace>>,
    pub legalities: Legalities,
    pub games: Vec<String>,                     // TODO: enum??
    pub reserved: bool,
    pub foil: bool,
    pub nonfoil: bool,
    pub finishes: Vec<String>,                  // TODO: enum??
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
    pub rarity: String,                         // TODO: enum
    pub card_back_id: Option<String>,           // TODO: uuid
    pub artist: String,
    pub artist_ids: Vec<String>,                // TODO: uuid
    pub illustration_id: Option<String>,        // TODO: uuid
    pub border_color: String,
    pub frame: String,
    pub security_stamp: Option<String>,
    pub full_art: bool,
    pub textless: bool,
    pub booster: bool,
    pub story_spotlight: bool,
    pub edhrec_rank: Option<i64>,
    pub penny_rank: Option<i64>,
    pub prices: Prices,
    pub related_uris: Option<RelatedUris>,
    pub purchase_uris: Option<PurchaseUris>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CardPage {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub total_cards: i64,
    pub has_more: bool,
    pub next_page: Option<Url>,
    pub data: Vec<Card>,
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
    pub artist_id: Option<String>,              // TODO: uuid
    pub illustration_id: Option<String>,        // TODO: uuid
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
    pub gatherer: Option<Url>,
    pub tcgplayer_infinite_articles: Option<Url>,
    pub tcgplayer_infinite_decks: Option<Url>,
    pub edhrec: Option<Url>,
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
    pub q: String,
    pub unique: Option<UniqueMode>,
    pub order: Option<OrderField>,
    pub dir: Option<OrderDirection>,
    pub include_extras: Option<bool>,
    pub include_multilingual: Option<bool>,
    pub include_variations: Option<bool>,
    pub page: Option<u32>,
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
