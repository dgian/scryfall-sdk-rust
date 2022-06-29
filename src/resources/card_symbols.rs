//! Card symbols resource definitions
//!
//! See [Scryfall api documentation](https://scryfall.com/docs/api/card-symbols)

use reqwest::Method;
use serde::{Deserialize, Serialize};
use url::Url;
use crate::HttpResource;
use crate::resources::ResourceKind;

/// Binding for endpoint `GET /symbology`
pub struct CardSymbolsResource;

/// Binding for endpoint `GET /symbology/parse-mana?cost=x`
pub struct ManaCostResource<'a>(pub &'a str);

impl HttpResource<CardSymbolList> for CardSymbolsResource {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        "symbology".into()
    }
}

impl<'a> HttpResource<ManaCost> for ManaCostResource<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("symbology/parse-mana?cost={}", self.0)
    }
}

/// Basic struct representing card symbol list
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CardSymbolList {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub has_more: bool,
    pub data: Vec<CardSymbol>,
}

/// A card symbol
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CardSymbol {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub symbol: String,
    pub svg_uri: Url,
    pub loose_variant: Option<String>,
    pub english: String,
    pub transposable: bool,
    pub represents_mana: bool,
    pub appears_in_mana_costs: bool,
    pub cmc: Option<f64>,
    pub funny: bool,
    pub colors: Vec<ColorSymbol>,
    pub gatherer_alternates: Option<Vec<String>>,
}

/// A mana cost entry
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ManaCost {
    #[serde(rename = "object")]
    pub kind: ResourceKind,
    pub cost: String,
    pub colors: Vec<ColorSymbol>,
    pub cmc: i64,
    pub colorless: bool,
    pub monocolored: bool,
    pub multicolored: bool,
}

/// A color symbol (single color)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ColorSymbol {
    /// Black
    B,

    /// Green
    G,

    /// Red
    R,

    /// Blue
    U,

    /// White
    W,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mana_cost_resource_should_return_path_and_method() {
        let resource = ManaCostResource("test");

        assert_eq!("symbology/parse-mana?cost=test", resource.path());
        assert_eq!(Method::GET, resource.method());
    }
}
