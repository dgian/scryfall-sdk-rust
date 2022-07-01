use httpmock::Method::GET;
use httpmock::MockServer;
use indoc::indoc;
use rstest::{fixture, rstest};
use time::macros::date;
use url::Url;

use scryfall_sdk_rust::{
    HttpResource,
    resources::{
        ResourceKind,
        card_symbols::ColorSymbol,
        cards::{Card, CardFace, ImageUris, Legalities, Legality, Prices, PurchaseUris, RelatedUris}
    },
    Scryfall,
    ScryfallBlocking,
    CardResource,
};

// -- CardResource tests
mod single {
    use super::*;

    #[fixture]
    #[once]
    fn response() -> String {
        indoc!(r#"
        {
          "object": "card",
          "id": "f295b713-1d6a-43fd-910d-fb35414bf58a",
          "oracle_id": "7bc3f92f-68a2-4934-afc4-89f6d0e8cf98",
          "multiverse_ids": [
            567508
          ],
          "tcgplayer_id": 273737,
          "name": "Dusk // Dawn",
          "lang": "en",
          "released_at": "2022-06-10",
          "uri": "http://some.url",
          "scryfall_uri": "http://some.url",
          "layout": "split",
          "highres_image": false,
          "image_status": "lowres",
          "image_uris": {
            "small": "http://some.url",
            "normal": "http://some.url",
            "large": "http://some.url",
            "png": "http://some.url",
            "art_crop": "http://some.url",
            "border_crop": "http://some.url"
          },
          "mana_cost": "{2}{W}{W} // {3}{W}{W}",
          "cmc": 9,
          "type_line": "Sorcery // Sorcery",
          "colors": [
            "W"
          ],
          "color_identity": [
            "W"
          ],
          "keywords": [
            "Aftermath"
          ],
          "card_faces": [
            {
              "object": "card_face",
              "name": "Dusk",
              "mana_cost": "{2}{W}{W}",
              "type_line": "Sorcery",
              "oracle_text": "Destroy all creatures with power 3 or greater.",
              "artist": "Kasia 'Kafis' Zielińska",
              "artist_id": "a662cb71-4770-4b49-8b03-2cf8497049a7",
              "illustration_id": "3134f77c-7a7d-48e0-99a6-4f323868e1ef"
            }
          ],
          "legalities": {
            "standard": "not_legal",
            "future": "not_legal",
            "historic": "legal",
            "gladiator": "legal",
            "pioneer": "legal",
            "explorer": "legal",
            "modern": "legal",
            "legacy": "legal",
            "pauper": "not_legal",
            "vintage": "legal",
            "penny": "legal",
            "commander": "legal",
            "brawl": "not_legal",
            "historicbrawl": "legal",
            "alchemy": "not_legal",
            "paupercommander": "not_legal",
            "duel": "legal",
            "oldschool": "not_legal",
            "premodern": "not_legal"
          },
          "games": [
            "paper"
          ],
          "reserved": false,
          "foil": false,
          "nonfoil": true,
          "finishes": [
            "nonfoil"
          ],
          "oversized": false,
          "promo": false,
          "reprint": true,
          "variation": false,
          "set_id": "5e4c3fe8-fd57-4b20-ad56-c03790a16cea",
          "set": "clb",
          "set_name": "Commander Legends: Battle for Baldur's Gate",
          "set_type": "draft_innovation",
          "set_uri": "http://some.url",
          "set_search_uri": "http://some.url",
          "scryfall_set_uri": "http://some.url",
          "rulings_uri": "http://some.url",
          "prints_search_uri": "http://some.url",
          "collector_number": "691",
          "digital": false,
          "rarity": "rare",
          "card_back_id": "0aeebaf5-8c7d-4636-9e82-8c27447861f7",
          "artist": "Kasia 'Kafis' Zielińska",
          "artist_ids": [
            "a662cb71-4770-4b49-8b03-2cf8497049a7"
          ],
          "illustration_id": "3134f77c-7a7d-48e0-99a6-4f323868e1ef",
          "border_color": "black",
          "frame": "2015",
          "security_stamp": "oval",
          "full_art": false,
          "textless": false,
          "booster": false,
          "story_spotlight": false,
          "edhrec_rank": 904,
          "penny_rank": 2681,
          "prices": {
            "usd": "0.13",
            "usd_foil": null,
            "usd_etched": null,
            "eur": null,
            "eur_foil": null,
            "tix": null
          },
          "related_uris": {
            "gatherer": "http://some.url",
            "tcgplayer_infinite_articles": "http://some.url",
            "tcgplayer_infinite_decks": "http://some.url",
            "edhrec": "http://some.url"
          },
          "purchase_uris": {
            "tcgplayer": "http://some.url",
            "cardmarket": "http://some.url",
            "cardhoarder": "http://some.url"
          }
        }
        "#).into()
    }

    #[fixture]
    #[once]
    fn card() -> Card {
        Card {
            kind: ResourceKind::Card,
            id: "f295b713-1d6a-43fd-910d-fb35414bf58a".into(),
            oracle_id: "7bc3f92f-68a2-4934-afc4-89f6d0e8cf98".into(),
            multiverse_ids: vec![567508],
            tcgplayer_id: Some(273737),
            name: "Dusk // Dawn".into(),
            lang: "en".into(),
            released_at: date!(2022-06-10),
            uri: "http://some.url".parse::<Url>().unwrap(),
            scryfall_uri: "http://some.url".parse::<Url>().unwrap(),
            layout: "split".into(),
            highres_image: false,
            image_status: "lowres".into(),
            image_uris: Some(ImageUris {
                small: "http://some.url".parse::<Url>().unwrap(),
                normal: "http://some.url".parse::<Url>().unwrap(),
                large: "http://some.url".parse::<Url>().unwrap(),
                png: "http://some.url".parse::<Url>().unwrap(),
                art_crop: "http://some.url".parse::<Url>().unwrap(),
                border_crop: "http://some.url".parse::<Url>().unwrap(),
            }),
            mana_cost: Some("{2}{W}{W} // {3}{W}{W}".into()),
            cmc: 9.0,
            type_line: "Sorcery // Sorcery".into(),
            colors: Some(vec![ColorSymbol::W]),
            color_identity: vec![ColorSymbol::W],
            keywords: vec!["Aftermath".into()],
            card_faces: Some(vec![
                CardFace {
                    kind: ResourceKind::CardFace,
                    name: "Dusk".into(),
                    mana_cost: "{2}{W}{W}".into(),
                    type_line: "Sorcery".into(),
                    oracle_text: "Destroy all creatures with power 3 or greater.".into(),
                    artist: Some("Kasia 'Kafis' Zielińska".into()),
                    artist_id: Some("a662cb71-4770-4b49-8b03-2cf8497049a7".into()),
                    illustration_id: Some("3134f77c-7a7d-48e0-99a6-4f323868e1ef".into()),
                    flavor_name: None,
                }
            ]),
            legalities: Legalities {
                standard: Legality::NotLegal,
                future: Legality::NotLegal,
                historic: Legality::Legal,
                gladiator: Legality::Legal,
                pioneer: Legality::Legal,
                explorer: Legality::Legal,
                modern: Legality::Legal,
                legacy: Legality::Legal,
                pauper: Legality::NotLegal,
                vintage: Legality::Legal,
                penny: Legality::Legal,
                commander: Legality::Legal,
                brawl: Legality::NotLegal,
                historicbrawl: Legality::Legal,
                alchemy: Legality::NotLegal,
                paupercommander: Legality::NotLegal,
                duel: Legality::Legal,
                oldschool: Legality::NotLegal,
                premodern: Legality::NotLegal,
            },
            games: vec!["paper".into()],
            reserved: false,
            foil: false,
            nonfoil: true,
            finishes: vec!["nonfoil".into()],
            oversized: false,
            promo: false,
            reprint: true,
            variation: false,
            set_id: "5e4c3fe8-fd57-4b20-ad56-c03790a16cea".into(),
            set: "clb".into(),
            set_name: "Commander Legends: Battle for Baldur's Gate".into(),
            set_type: "draft_innovation".into(),
            set_uri: "http://some.url".parse::<Url>().unwrap(),
            set_search_uri: "http://some.url".parse::<Url>().unwrap(),
            scryfall_set_uri: "http://some.url".parse::<Url>().unwrap(),
            rulings_uri: "http://some.url".parse::<Url>().unwrap(),
            prints_search_uri: "http://some.url".parse::<Url>().unwrap(),
            collector_number: "691".into(),
            digital: false,
            rarity: "rare".into(),
            card_back_id: Some("0aeebaf5-8c7d-4636-9e82-8c27447861f7".into()),
            artist: "Kasia 'Kafis' Zielińska".into(),
            artist_ids: vec!["a662cb71-4770-4b49-8b03-2cf8497049a7".into()],
            illustration_id: Some("3134f77c-7a7d-48e0-99a6-4f323868e1ef".into()),
            border_color: "black".into(),
            frame: "2015".into(),
            security_stamp: Some("oval".into()),
            full_art: false,
            textless: false,
            booster: false,
            story_spotlight: false,
            edhrec_rank: Some(904),
            penny_rank: Some(2681),
            prices: Prices {
                usd: Some("0.13".into()),
                usd_foil: None,
                usd_etched: None,
                eur: None,
                eur_foil: None,
                tix: None,
            },
            related_uris: Some(RelatedUris {
                gatherer: Some("http://some.url".parse::<Url>().unwrap()),
                tcgplayer_infinite_articles: Some("http://some.url".parse::<Url>().unwrap()),
                tcgplayer_infinite_decks: Some("http://some.url".parse::<Url>().unwrap()),
                edhrec: Some("http://some.url".parse::<Url>().unwrap()),
            }),
            purchase_uris: Some(PurchaseUris {
                tcgplayer: "http://some.url".parse::<Url>().unwrap(),
                cardmarket: "http://some.url".parse::<Url>().unwrap(),
                cardhoarder: "http://some.url".parse::<Url>().unwrap(),
            })
        }
    }

    #[rstest]
    #[case::by_id(CardResource::ById("123"))]
    #[case::by_arena_id(CardResource::ByArenaId("123"))]
    #[case::by_cardmarket_id(CardResource::ByCardmarketId("123"))]
    #[case::by_code(CardResource::ByCode("123", "456"))]
    #[case::by_mtgo_id(CardResource::ByMtgoId("123"))]
    #[case::by_multiverse_id(CardResource::ByMultiverseId("123"))]
    #[case::by_tcgplayer_id(CardResource::ByTcgplayerId("123"))]
    #[case::named_exact(CardResource::NamedExact("exact"))]
    #[case::named_fuzzy(CardResource::NamedFuzzy("fuzzy"))]
    #[case::random(CardResource::Random(None))]
    #[case::random_with_name(CardResource::Random(Some("name")))]
    fn test_blocking_request(response: &String, card: &Card, #[case] resource: CardResource) {
        let server = MockServer::start();

            let endpoint = server.mock(|when, then| {
                when.method(GET).path(format!("/{}", resource.path_without_query()));
                then.status(200)
                    .header("content-type", "application/json")
                    .body(response);
            });

        let url = server.base_url();
        let client = ScryfallBlocking::from_url(&url);

        let response = client
            .request(&resource)
            .expect("Expected a valid Card response");

        endpoint.assert();
        assert_eq!(card, &response)
    }

    #[rstest]
    #[case::by_id(CardResource::ById("123"))]
    #[case::by_arena_id(CardResource::ByArenaId("123"))]
    #[case::by_cardmarket_id(CardResource::ByCardmarketId("123"))]
    #[case::by_code(CardResource::ByCode("123", "456"))]
    #[case::by_mtgo_id(CardResource::ByMtgoId("123"))]
    #[case::by_multiverse_id(CardResource::ByMultiverseId("123"))]
    #[case::by_tcgplayer_id(CardResource::ByTcgplayerId("123"))]
    #[case::named_exact(CardResource::NamedExact("exact"))]
    #[case::named_fuzzy(CardResource::NamedFuzzy("fuzzy"))]
    #[case::random(CardResource::Random(None))]
    #[case::random_with_name(CardResource::Random(Some("name")))]
    #[tokio::test]
    async fn test_async_request<'a>(response: &String, card: &Card, #[case] resource: CardResource<'a>) {
        let server = MockServer::start_async().await;

        let endpoint = server.mock(|when, then| {
            when.method(GET).path(format!("/{}", resource.path_without_query()));
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });

        let url = server.base_url();
        let client = Scryfall::from_url(&url);

        let response = client
            .request(&resource).await
            .expect("Expected a valid Card response");

        endpoint.assert();
        assert_eq!(card, &response)
    }
}

// -- CardPageResource tests
mod page {
    use scryfall_sdk_rust::resources::cards::{CardPage, CardPageResource, SearchQueryParams};
    use super::*;

    #[fixture]
    #[once]
    fn response() -> String {
        indoc!(r#"
        {
            "object": "list",
            "total_cards": 1,
            "has_more": false,
            "next_page": "http://some.url",
            "data": [{
              "object": "card",
              "id": "f295b713-1d6a-43fd-910d-fb35414bf58a",
              "oracle_id": "7bc3f92f-68a2-4934-afc4-89f6d0e8cf98",
              "multiverse_ids": [
                567508
              ],
              "tcgplayer_id": 273737,
              "name": "Dusk // Dawn",
              "lang": "en",
              "released_at": "2022-06-10",
              "uri": "http://some.url",
              "scryfall_uri": "http://some.url",
              "layout": "split",
              "highres_image": false,
              "image_status": "lowres",
              "image_uris": {
                "small": "http://some.url",
                "normal": "http://some.url",
                "large": "http://some.url",
                "png": "http://some.url",
                "art_crop": "http://some.url",
                "border_crop": "http://some.url"
              },
              "mana_cost": "{2}{W}{W} // {3}{W}{W}",
              "cmc": 9,
              "type_line": "Sorcery // Sorcery",
              "colors": [
                "W"
              ],
              "color_identity": [
                "W"
              ],
              "keywords": [
                "Aftermath"
              ],
              "card_faces": [
                {
                  "object": "card_face",
                  "name": "Dusk",
                  "mana_cost": "{2}{W}{W}",
                  "type_line": "Sorcery",
                  "oracle_text": "Destroy all creatures with power 3 or greater.",
                  "artist": "Kasia 'Kafis' Zielińska",
                  "artist_id": "a662cb71-4770-4b49-8b03-2cf8497049a7",
                  "illustration_id": "3134f77c-7a7d-48e0-99a6-4f323868e1ef"
                }
              ],
              "legalities": {
                "standard": "not_legal",
                "future": "not_legal",
                "historic": "legal",
                "gladiator": "legal",
                "pioneer": "legal",
                "explorer": "legal",
                "modern": "legal",
                "legacy": "legal",
                "pauper": "not_legal",
                "vintage": "legal",
                "penny": "legal",
                "commander": "legal",
                "brawl": "not_legal",
                "historicbrawl": "legal",
                "alchemy": "not_legal",
                "paupercommander": "not_legal",
                "duel": "legal",
                "oldschool": "not_legal",
                "premodern": "not_legal"
              },
              "games": [
                "paper"
              ],
              "reserved": false,
              "foil": false,
              "nonfoil": true,
              "finishes": [
                "nonfoil"
              ],
              "oversized": false,
              "promo": false,
              "reprint": true,
              "variation": false,
              "set_id": "5e4c3fe8-fd57-4b20-ad56-c03790a16cea",
              "set": "clb",
              "set_name": "Commander Legends: Battle for Baldur's Gate",
              "set_type": "draft_innovation",
              "set_uri": "http://some.url",
              "set_search_uri": "http://some.url",
              "scryfall_set_uri": "http://some.url",
              "rulings_uri": "http://some.url",
              "prints_search_uri": "http://some.url",
              "collector_number": "691",
              "digital": false,
              "rarity": "rare",
              "card_back_id": "0aeebaf5-8c7d-4636-9e82-8c27447861f7",
              "artist": "Kasia 'Kafis' Zielińska",
              "artist_ids": [
                "a662cb71-4770-4b49-8b03-2cf8497049a7"
              ],
              "illustration_id": "3134f77c-7a7d-48e0-99a6-4f323868e1ef",
              "border_color": "black",
              "frame": "2015",
              "security_stamp": "oval",
              "full_art": false,
              "textless": false,
              "booster": false,
              "story_spotlight": false,
              "edhrec_rank": 904,
              "penny_rank": 2681,
              "prices": {
                "usd": "0.13",
                "usd_foil": null,
                "usd_etched": null,
                "eur": null,
                "eur_foil": null,
                "tix": null
              },
              "related_uris": {
                "gatherer": "http://some.url",
                "tcgplayer_infinite_articles": "http://some.url",
                "tcgplayer_infinite_decks": "http://some.url",
                "edhrec": "http://some.url"
              },
              "purchase_uris": {
                "tcgplayer": "http://some.url",
                "cardmarket": "http://some.url",
                "cardhoarder": "http://some.url"
              }
            }]
        }
        "#).into()
    }

    #[fixture]
    #[once]
    fn card_page() -> CardPage {
        CardPage {
            kind: ResourceKind::List,
            total_cards: 1,
            has_more: false,
            next_page: Some("http://some.url".parse::<Url>().unwrap()),
            data: vec![
                Card {
                    kind: ResourceKind::Card,
                    id: "f295b713-1d6a-43fd-910d-fb35414bf58a".into(),
                    oracle_id: "7bc3f92f-68a2-4934-afc4-89f6d0e8cf98".into(),
                    multiverse_ids: vec![567508],
                    tcgplayer_id: Some(273737),
                    name: "Dusk // Dawn".into(),
                    lang: "en".into(),
                    released_at: date!(2022-06-10),
                    uri: "http://some.url".parse::<Url>().unwrap(),
                    scryfall_uri: "http://some.url".parse::<Url>().unwrap(),
                    layout: "split".into(),
                    highres_image: false,
                    image_status: "lowres".into(),
                    image_uris: Some(ImageUris {
                        small: "http://some.url".parse::<Url>().unwrap(),
                        normal: "http://some.url".parse::<Url>().unwrap(),
                        large: "http://some.url".parse::<Url>().unwrap(),
                        png: "http://some.url".parse::<Url>().unwrap(),
                        art_crop: "http://some.url".parse::<Url>().unwrap(),
                        border_crop: "http://some.url".parse::<Url>().unwrap(),
                    }),
                    mana_cost: Some("{2}{W}{W} // {3}{W}{W}".into()),
                    cmc: 9.0,
                    type_line: "Sorcery // Sorcery".into(),
                    colors: Some(vec![ColorSymbol::W]),
                    color_identity: vec![ColorSymbol::W],
                    keywords: vec!["Aftermath".into()],
                    card_faces: Some(vec![
                        CardFace {
                            kind: ResourceKind::CardFace,
                            name: "Dusk".into(),
                            mana_cost: "{2}{W}{W}".into(),
                            type_line: "Sorcery".into(),
                            oracle_text: "Destroy all creatures with power 3 or greater.".into(),
                            artist: Some("Kasia 'Kafis' Zielińska".into()),
                            artist_id: Some("a662cb71-4770-4b49-8b03-2cf8497049a7".into()),
                            illustration_id: Some("3134f77c-7a7d-48e0-99a6-4f323868e1ef".into()),
                            flavor_name: None,
                        }
                    ]),
                    legalities: Legalities {
                        standard: Legality::NotLegal,
                        future: Legality::NotLegal,
                        historic: Legality::Legal,
                        gladiator: Legality::Legal,
                        pioneer: Legality::Legal,
                        explorer: Legality::Legal,
                        modern: Legality::Legal,
                        legacy: Legality::Legal,
                        pauper: Legality::NotLegal,
                        vintage: Legality::Legal,
                        penny: Legality::Legal,
                        commander: Legality::Legal,
                        brawl: Legality::NotLegal,
                        historicbrawl: Legality::Legal,
                        alchemy: Legality::NotLegal,
                        paupercommander: Legality::NotLegal,
                        duel: Legality::Legal,
                        oldschool: Legality::NotLegal,
                        premodern: Legality::NotLegal,
                    },
                    games: vec!["paper".into()],
                    reserved: false,
                    foil: false,
                    nonfoil: true,
                    finishes: vec!["nonfoil".into()],
                    oversized: false,
                    promo: false,
                    reprint: true,
                    variation: false,
                    set_id: "5e4c3fe8-fd57-4b20-ad56-c03790a16cea".into(),
                    set: "clb".into(),
                    set_name: "Commander Legends: Battle for Baldur's Gate".into(),
                    set_type: "draft_innovation".into(),
                    set_uri: "http://some.url".parse::<Url>().unwrap(),
                    set_search_uri: "http://some.url".parse::<Url>().unwrap(),
                    scryfall_set_uri: "http://some.url".parse::<Url>().unwrap(),
                    rulings_uri: "http://some.url".parse::<Url>().unwrap(),
                    prints_search_uri: "http://some.url".parse::<Url>().unwrap(),
                    collector_number: "691".into(),
                    digital: false,
                    rarity: "rare".into(),
                    card_back_id: Some("0aeebaf5-8c7d-4636-9e82-8c27447861f7".into()),
                    artist: "Kasia 'Kafis' Zielińska".into(),
                    artist_ids: vec!["a662cb71-4770-4b49-8b03-2cf8497049a7".into()],
                    illustration_id: Some("3134f77c-7a7d-48e0-99a6-4f323868e1ef".into()),
                    border_color: "black".into(),
                    frame: "2015".into(),
                    security_stamp: Some("oval".into()),
                    full_art: false,
                    textless: false,
                    booster: false,
                    story_spotlight: false,
                    edhrec_rank: Some(904),
                    penny_rank: Some(2681),
                    prices: Prices {
                        usd: Some("0.13".into()),
                        usd_foil: None,
                        usd_etched: None,
                        eur: None,
                        eur_foil: None,
                        tix: None,
                    },
                    related_uris: Some(RelatedUris {
                        gatherer: Some("http://some.url".parse::<Url>().unwrap()),
                        tcgplayer_infinite_articles: Some("http://some.url".parse::<Url>().unwrap()),
                        tcgplayer_infinite_decks: Some("http://some.url".parse::<Url>().unwrap()),
                        edhrec: Some("http://some.url".parse::<Url>().unwrap()),
                    }),
                    purchase_uris: Some(PurchaseUris {
                        tcgplayer: "http://some.url".parse::<Url>().unwrap(),
                        cardmarket: "http://some.url".parse::<Url>().unwrap(),
                        cardhoarder: "http://some.url".parse::<Url>().unwrap(),
                    }),
                }],
        }
    }

    #[rstest]
    fn test_blocking_request(response: &String, card_page: &CardPage) {
        let server = MockServer::start();

        let resource = CardPageResource::Search(SearchQueryParams {
            q: "test".into(),
            unique: None,
            order: None,
            dir: None,
            include_variations: None,
            include_multilingual: None,
            include_extras: None,
            page: None,
        });

        let endpoint = server.mock(|when, then| {
                when.method(GET).path(format!("/{}", resource.path_without_query()));
                then.status(200)
                    .header("content-type", "application/json")
                    .body(response);
            });

        let url = server.base_url();
        let client = ScryfallBlocking::from_url(&url);

        let response = client
            .request(&resource)
            .expect("Expected a valid CardPage response");

        endpoint.assert();
        assert_eq!(card_page, &response)
    }

    #[rstest]
    #[tokio::test]
    async fn test_async_request<'a>(response: &String, card_page: &CardPage) {
        let server = MockServer::start_async().await;

        let resource = CardPageResource::Search(SearchQueryParams {
            q: "test".into(),
            unique: None,
            order: None,
            dir: None,
            include_variations: None,
            include_multilingual: None,
            include_extras: None,
            page: None,
        });

        let endpoint = server.mock(|when, then| {
            when.method(GET).path(format!("/{}", resource.path_without_query()));
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });

        let url = server.base_url();
        let client = Scryfall::from_url(&url);

        let response = client
            .request(&resource).await
            .expect("Expected a valid CardPage response");

        endpoint.assert();
        assert_eq!(card_page, &response)
    }
}
