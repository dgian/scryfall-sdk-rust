//! Ruling resource definitions
//!
//! See [Scryfall api documentation](https://scryfall.com/docs/api/rulings)

use reqwest::Method;
use serde::{Deserialize, Serialize};
use time::Date;

use RulingListResource::{ByArenaId, ByCardId, ByMtgoId, ByMultiverseId, BySetCode};

use crate::HttpResource;
use crate::resources::ResourceKind;

/// Endpoints for `/cards/**/rulings` resource
pub enum RulingListResource<'a> {
    /// Binding for endpoint `GET /cards/:id/rulings`
    ///
    /// Get ruling list by card's scryfall id.
    ByCardId(&'a str),

    /// Binding for endpoint `GET /cards/:code/:number/rulings`
    ///
    /// Get ruling list by card set and card number.
    BySetCode(&'a str, i32),

    /// Binding for endpoint `GET /cards/arena/:id/rulings`
    ///
    /// Get ruling list by card's arena id.
    ByArenaId(i32),

    /// Binding for endpoint `GET /cards/mtgo/:id/rulings`
    ///
    /// Get ruling list by card's MTGO id.
    ByMtgoId(i32),

    /// Binding for endpoint `GET /cards/multiverse/:id/rulings`
    ///
    /// Get ruling list by card's multiverse id.
    ByMultiverseId(i32),
}

impl<'a> HttpResource<RulingList> for RulingListResource<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("cards/{}/rulings", match self {
            ByCardId(id) => format!("{id}"),
            BySetCode(code, num) => format!("{code}/{num}"),
            ByArenaId(id) => format!("arena/{id}"),
            ByMtgoId(id) => format!("mtgo/{id}"),
            ByMultiverseId(id) => format!("multiverse/{id}"),
        })
    }
}

/// Basic struct representing ruling list
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RulingList {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub has_more: bool,
    pub data: Vec<Ruling>,
}

/// A ruling
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ruling {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub oracle_id: String,
    pub source: String,
    pub published_at: Date,
    pub comment: String,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::by_card_id(RulingListResource::ByCardId("id"), "cards/id/rulings")]
    #[case::by_set_code(RulingListResource::BySetCode("code", 123), "cards/code/123/rulings")]
    #[case::by_arena_id(RulingListResource::ByArenaId(123), "cards/arena/123/rulings")]
    #[case::by_mtgo_id(RulingListResource::ByMtgoId(123), "cards/mtgo/123/rulings")]
    #[case::by_multiverse_id(RulingListResource::ByMultiverseId(123), "cards/multiverse/123/rulings")]
    fn ruling_list_resource_should_return_path_and_method(
        #[case] resource: RulingListResource,
        #[case] expected: &str
    ) {
        assert_eq!(expected, resource.path());
        assert_eq!(Method::GET, resource.method());
    }
}
