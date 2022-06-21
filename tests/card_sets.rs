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
        card_sets::{CardSet, SetKind, CardSetList}
    },
    Scryfall,
    ScryfallBlocking,
    CardSetListResource,
    CardSetResource
};

// -- CardSetListResource tests
mod list {
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
              "object": "set",
              "id": "4219a14e-6701-4ddd-a185-21dc054ab19b",
              "code": "bro",
              "mtgo_code": "bro",
              "arena_code": "bro",
              "name": "The Brothers' War",
              "uri": "https://some-url.com",
              "scryfall_uri": "https://some-url.com",
              "search_uri": "https://some-url.com",
              "released_at": "2022-11-18",
              "set_type": "expansion",
              "card_count": 0,
              "digital": false,
              "nonfoil_only": true,
              "foil_only": true,
              "icon_svg_uri": "https://some-url.com"
            },
            {
              "object": "set",
              "id": "b314f553-8f07-4ba9-96c8-16be7784eff3",
              "code": "unf",
              "tcgplayer_id": 2958,
              "name": "Unfinity",
              "uri": "https://some-url.com",
              "scryfall_uri": "https://some-url.com",
              "search_uri": "https://some-url.com",
              "released_at": "2022-10-07",
              "set_type": "funny",
              "card_count": 26,
              "digital": false,
              "nonfoil_only": false,
              "foil_only": false,
              "icon_svg_uri": "https://some-url.com",
              "parent_set_code": "parent",
              "block_code": "block_code",
              "block": "block"

            }
          ]
        }
        "#).into()
    }

    #[fixture]
    #[once]
    fn card_set_list() -> CardSetList {
        CardSetList {
            kind: ResourceKind::List,
            has_more: false,
            data: vec![CardSet {
                item_kind: ResourceKind::Set,
                id: "4219a14e-6701-4ddd-a185-21dc054ab19b".into(),
                code: "bro".into(),
                tcgplayer_id: None,
                mtgo_code: Some("bro".into()),
                arena_code: Some("bro".into()),
                name: "The Brothers' War".into(),
                uri: "https://some-url.com".parse::<Url>().unwrap(),
                scryfall_uri: "https://some-url.com".parse::<Url>().unwrap(),
                search_uri: "https://some-url.com".parse::<Url>().unwrap(),
                released_at: date!(2022-11-18),
                kind: SetKind::Expansion,
                card_count: 0,
                digital: false,
                nonfoil_only: true,
                foil_only: true,
                icon_svg_uri: "https://some-url.com".parse::<Url>().unwrap(),
                parent_set_code: None,
                block_code: None,
                block: None,
            },
                       CardSet {
                item_kind: ResourceKind::Set,
                id: "b314f553-8f07-4ba9-96c8-16be7784eff3".into(),
                code: "unf".into(),
                tcgplayer_id: Some(2958),
                mtgo_code: None,
                arena_code: None,
                name: "Unfinity".into(),
                uri: "https://some-url.com".parse::<Url>().unwrap(),
                scryfall_uri: "https://some-url.com".parse::<Url>().unwrap(),
                search_uri: "https://some-url.com".parse::<Url>().unwrap(),
                released_at: date!(2022-10-07),
                kind: SetKind::Funny,
                card_count: 26,
                digital: false,
                nonfoil_only: false,
                foil_only: false,
                icon_svg_uri: "https://some-url.com".parse::<Url>().unwrap(),
                parent_set_code: Some("parent".into()),
                block_code: Some("block_code".into()),
                block: Some("block".into()),
            }]
        }
    }

    #[rstest]
    fn test_blocking_request(response: &String, card_set_list: &CardSetList) {
        let server = MockServer::start();

            let endpoint = server.mock(|when, then| {
                when.method(GET).path("/sets");
                then.status(200)
                    .header("content-type", "application/json")
                    .body(response);
            });

        let url = server.base_url();
        let client = ScryfallBlocking::from_url(&url);

        let response = client
            .request(&CardSetListResource::All)
            .expect("Expected a valid CardSetList response");

        endpoint.assert();
        assert_eq!(card_set_list, &response)
    }

    #[rstest]
    #[tokio::test]
    async fn test_async_request(response: &String, card_set_list: &CardSetList) {
        let server = MockServer::start_async().await;

        let endpoint = server.mock(|when, then| {
            when.method(GET).path("/sets");
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });

        let url = server.base_url();
        let client = Scryfall::from_url(&url);

        let response = client
            .request(&CardSetListResource::All).await
            .expect("Expected a valid CardSetList response");

        endpoint.assert();
        assert_eq!(card_set_list, &response)
    }
}

// -- CardSetResource tests
mod single {
    use super::*;

    #[fixture]
    #[once]
    fn response() -> String {
        indoc!(r#"
        {
          "object": "set",
          "id": "4219a14e-6701-4ddd-a185-21dc054ab19b",
          "code": "bro",
          "mtgo_code": "bro",
          "arena_code": "bro",
          "name": "The Brothers' War",
          "uri": "https://some-url.com",
          "scryfall_uri": "https://some-url.com",
          "search_uri": "https://some-url.com",
          "released_at": "2022-11-18",
          "set_type": "expansion",
          "card_count": 0,
          "digital": false,
          "nonfoil_only": true,
          "foil_only": true,
          "icon_svg_uri": "https://some-url.com"
        }
        "#).into()
    }

    #[fixture]
    #[once]
    fn card_set() -> CardSet {
        CardSet {
            item_kind: ResourceKind::Set,
            id: "4219a14e-6701-4ddd-a185-21dc054ab19b".into(),
            code: "bro".into(),
            tcgplayer_id: None,
            mtgo_code: Some("bro".into()),
            arena_code: Some("bro".into()),
            name: "The Brothers' War".into(),
            uri: "https://some-url.com".parse::<Url>().unwrap(),
            scryfall_uri: "https://some-url.com".parse::<Url>().unwrap(),
            search_uri: "https://some-url.com".parse::<Url>().unwrap(),
            released_at: date!(2022-11-18),
            kind: SetKind::Expansion,
            card_count: 0,
            digital: false,
            nonfoil_only: true,
            foil_only: true,
            icon_svg_uri: "https://some-url.com".parse::<Url>().unwrap(),
            parent_set_code: None,
            block_code: None,
            block: None,
        }
    }

    #[rstest]
    #[case::filter(CardSetResource::Filter("id"))]
    #[case::with_tcgplayerid(CardSetResource::WithTcgPlayerId("id"))]
    fn test_blocking_request(
        response: &String,
        card_set: &CardSet,
        #[case]resource: CardSetResource
    ) {
        let server = MockServer::start();

            let endpoint = server.mock(|when, then| {
                when.method(GET).path(format!("/{}", resource.path()));
                then.status(200)
                    .header("content-type", "application/json")
                    .body(response);
            });

        let url = server.base_url();
        let client = ScryfallBlocking::from_url(&url);

        let response = client
            .request(&resource)
            .expect("Expected a valid CardSet response");

        endpoint.assert();
        assert_eq!(card_set, &response)
    }

    #[rstest]
    #[case::filter(CardSetResource::Filter("id"))]
    #[case::with_tcgplayerid(CardSetResource::WithTcgPlayerId("id"))]
    #[tokio::test]
    async fn test_async_request<'a>(
        response: &String,
        card_set: &CardSet,
        #[case]resource: CardSetResource<'a>
    ) {
        let server = MockServer::start_async().await;

        let endpoint = server.mock(|when, then| {
            when.method(GET).path(format!("/{}", resource.path()));
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });

        let url = server.base_url();
        let client = Scryfall::from_url(&url);

        let response = client
            .request(&resource).await
            .expect("Expected a valid CardSet response");

        endpoint.assert();
        assert_eq!(card_set, &response)
    }
}
