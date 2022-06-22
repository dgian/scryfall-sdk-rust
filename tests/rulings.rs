use httpmock::Method::GET;
use httpmock::MockServer;
use indoc::indoc;
use rstest::{fixture, rstest};
use time::macros::date;

use scryfall_sdk_rust::{
    HttpResource,
    resources::{
        ResourceKind,
        rulings::{Ruling, RulingList},
        rulings::RulingListResource
    },
    Scryfall,
    ScryfallBlocking,
};

#[fixture]
#[once]
fn response() -> String {
    indoc!(r#"
        {
          "object": "list",
          "has_more": false,
          "data": [
            {
              "object": "ruling",
              "oracle_id": "f5ca7b13-8003-4361-b827-7095c89f2750",
              "source": "wotc",
              "published_at": "2004-10-04",
              "comment": "It must flip like a coin and not like a Frisbee."
            }
          ]
        }
        "#).into()
}

#[fixture]
#[once]
fn ruling_list() -> RulingList {
    RulingList {
        kind: ResourceKind::List,
        has_more: false,
        data: vec![
            Ruling {
                kind: ResourceKind::Ruling,
                oracle_id: "f5ca7b13-8003-4361-b827-7095c89f2750".into(),
                source: "wotc".into(),
                published_at: date!(2004-10-04),
                comment: "It must flip like a coin and not like a Frisbee.".into(),
            },
        ],
    }
}

#[rstest]
#[case::by_card_id(RulingListResource::ByCardId("id"))]
#[case::by_set_code(RulingListResource::BySetCode("code", 123))]
#[case::by_arena_id(RulingListResource::ByArenaId(123))]
#[case::by_mtgo_id(RulingListResource::ByMtgoId(123))]
#[case::by_multiverse_id(RulingListResource::ByMultiverseId(123))]
fn test_blocking_request(
    response: &String,
    ruling_list: &RulingList,
    #[case]resource: RulingListResource
) {
    let server = MockServer::start();

    let endpoint = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/{}", resource.path()));

        then.status(200)
            .header("content-type", "application/json")
            .body(response);
    });

    let url = server.base_url();
    let client = ScryfallBlocking::from_url(&url);

    let response = client
        .request(&resource)
        .expect("Expected a valid RulingList response");

    endpoint.assert();
    assert_eq!(ruling_list, &response)
}

#[rstest]
#[case::by_card_id(RulingListResource::ByCardId("id"))]
#[case::by_set_code(RulingListResource::BySetCode("code", 123))]
#[case::by_arena_id(RulingListResource::ByArenaId(123))]
#[case::by_mtgo_id(RulingListResource::ByMtgoId(123))]
#[case::by_multiverse_id(RulingListResource::ByMultiverseId(123))]
#[tokio::test]
async fn test_async_request<'a>(
    response: &String,
    ruling_list: &RulingList,
    #[case]resource: RulingListResource<'a>,
) {
    let server = MockServer::start_async().await;

    let endpoint = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/{}", resource.path()));

        then.status(200)
            .header("content-type", "application/json")
            .body(response);
    });

    let url = server.base_url();
    let client = Scryfall::from_url(&url);

    let response = client
        .request(&resource).await
        .expect("Expected a valid RulingList response");

    endpoint.assert();
    assert_eq!(ruling_list, &response)
}
