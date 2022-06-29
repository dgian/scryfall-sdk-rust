use httpmock::Method::GET;
use httpmock::MockServer;
use indoc::indoc;
use rstest::{fixture, rstest};
use url::Url;

use scryfall_sdk_rust::{
    Scryfall,
    ScryfallBlocking,
    resources::{
        ResourceKind,
        card_symbols::{ColorSymbol, CardSymbol, CardSymbolList, ManaCost}
    },
    ManaCostResource,
    HttpResource
};

// -- SymbologyListResource tests
mod list {
    use scryfall_sdk_rust::{HttpResource, CardSymbolsResource};
    use super::*;

    #[fixture]
    #[once]
    fn response() -> String {
        indoc!(r#"
        {
          "object": "list",
          "has_more": false,
          "data": [
            {
              "object": "card_symbol",
              "symbol": "{T}",
              "svg_uri": "https://some-url.com",
              "loose_variant": null,
              "english": "tap this permanent",
              "transposable": false,
              "represents_mana": false,
              "appears_in_mana_costs": false,
              "cmc": 0,
              "funny": false,
              "colors": [],
              "gatherer_alternates": null
            },
            {
              "object": "card_symbol",
              "symbol": "{0}",
              "svg_uri": "https://some-url.com",
              "loose_variant": "0",
              "english": "zero mana",
              "transposable": false,
              "represents_mana": true,
              "appears_in_mana_costs": true,
              "cmc": 0,
              "funny": false,
              "colors": ["B", "G"],
              "gatherer_alternates": [
                "o0"
              ]
            }
          ]
        }
        "#).into()
    }

    #[fixture]
    #[once]
    fn symbology_list() -> CardSymbolList {
        CardSymbolList {
            kind: ResourceKind::List,
            has_more: false,
            data: vec![
                CardSymbol {
                    kind: ResourceKind::CardSymbol,
                    symbol: "{T}".into(),
                    svg_uri: "https://some-url.com".parse::<Url>().unwrap(),
                    loose_variant: None,
                    english: "tap this permanent".into(),
                    transposable: false,
                    represents_mana: false,
                    appears_in_mana_costs: false,
                    cmc: Some(0f64),
                    funny: false,
                    colors: vec![],
                    gatherer_alternates: None
                },
                CardSymbol {
                    kind: ResourceKind::CardSymbol,
                    symbol: "{0}".into(),
                    svg_uri: "https://some-url.com".parse::<Url>().unwrap(),
                    loose_variant: Some("0".into()),
                    english: "zero mana".into(),
                    transposable: false,
                    represents_mana: true,
                    appears_in_mana_costs: true,
                    cmc: Some(0f64),
                    funny: false,
                    colors: vec![
                        ColorSymbol::B,
                        ColorSymbol::G
                    ],
                    gatherer_alternates: Some(vec![
                        "o0".into()
                    ])
                }
            ]
        }
    }

    #[rstest]
    fn test_blocking_request(response: &String, symbology_list: &CardSymbolList) {
        let server = MockServer::start();

            let endpoint = server.mock(|when, then| {
                when.method(GET)
                    .path(format!("/{}", CardSymbolsResource.path()));

                then.status(200)
                    .header("content-type", "application/json")
                    .body(response);
            });

        let url = server.base_url();
        let client = ScryfallBlocking::from_url(&url);

        let response = client
            .request(&CardSymbolsResource)
            .expect("Expected a valid SymbologyList response");

        endpoint.assert();
        assert_eq!(symbology_list, &response)
    }

    #[rstest]
    #[tokio::test]
    async fn test_async_request(response: &String, symbology_list: &CardSymbolList) {
        let server = MockServer::start_async().await;

        let endpoint = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/{}", CardSymbolsResource.path()));

            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });

        let url = server.base_url();
        let client = Scryfall::from_url(&url);

        let response = client
            .request(&CardSymbolsResource).await
            .expect("Expected a valid BulkData response");

        endpoint.assert();
        assert_eq!(symbology_list, &response)
    }
}

// -- SymbologyManaResource tests
mod mana {
    use super::*;

    #[fixture]
    #[once]
    fn response() -> String {
        indoc!(r#"
        {
          "object": "mana_cost",
          "cost": "1UR",
          "colors": [
            "U",
            "R"
          ],
          "cmc": 1,
          "colorless": false,
          "monocolored": false,
          "multicolored": true
        }
        "#).into()
    }

    #[fixture]
    #[once]
    fn symbology_mana() -> ManaCost {
        ManaCost {
            kind: ResourceKind::ManaCost,
            cmc: 1,
            cost: "1UR".into(),
            colors: vec![ColorSymbol::U, ColorSymbol::R],
            colorless: false,
            monocolored: false,
            multicolored: true,
        }
    }

    #[rstest]
    fn test_blocking_request(response: &String, symbology_mana: &ManaCost) {
        let server = MockServer::start();

            let endpoint = server.mock(|when, then| {
                when.method(GET)
                    .path(format!("/{}", ManaCostResource("1b").path_without_query()))
                    .query_param("cost", "1b");

                then.status(200)
                    .header("content-type", "application/json")
                    .body(response);
            });

        let url = server.base_url();
        let client = ScryfallBlocking::from_url(&url);

        let response = client
            .request(&ManaCostResource("1b"))
            .expect("Expected a valid SymbologyMana response");

        endpoint.assert();
        assert_eq!(symbology_mana, &response)
    }

    #[rstest]
    #[tokio::test]
    async fn test_async_request(response: &String, symbology_mana: &ManaCost) {
        let server = MockServer::start_async().await;

        let endpoint = server.mock(|when, then| {
            when.method(GET)
                .path(format!("/{}", ManaCostResource("1b").path_without_query()))
                .query_param("cost", "1b");

            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });

        let url = server.base_url();
        let client = Scryfall::from_url(&url);

        let response = client
            .request(&ManaCostResource("1b")).await
            .expect("Expected a valid BulkData response");

        endpoint.assert();
        assert_eq!(symbology_mana, &response)
    }
}
