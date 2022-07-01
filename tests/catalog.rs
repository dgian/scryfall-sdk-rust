use httpmock::Method::GET;
use httpmock::MockServer;
use indoc::indoc;
use rstest::{fixture, rstest};
use url::Url;

use scryfall_sdk_rust::{
    Scryfall,
    ScryfallBlocking,
    HttpResource,
    resources::{
        catalog::Catalog,
        catalog::CatalogResource,
        ResourceKind
    }
};

#[fixture]
#[once]
fn response() -> String {
    indoc!(r#"
    {
      "object": "catalog",
      "uri": "https://some-url.com",
      "total_values": 3,
      "data": [
        "SomeValue",
        "SomeValue",
        "SomeValue"
      ]
    }
    "#).into()
}

#[fixture]
#[once]
fn catalog() -> Catalog {
    Catalog {
        kind: ResourceKind::Catalog,
        uri: Some("https://some-url.com".parse::<Url>().unwrap()),
        total_values: 3,
        data: vec!["SomeValue".into(), "SomeValue".into(), "SomeValue".into()]
    }
}

#[rstest]
#[case::ability_words(CatalogResource::AbilityWords)]
#[case::artifact_types(CatalogResource::ArtifactTypes)]
#[case::artist_names(CatalogResource::ArtistNames)]
#[case::card_names(CatalogResource::CardNames)]
#[case::creature_types(CatalogResource::CreatureTypes)]
#[case::enchantment_types(CatalogResource::EnchantmentTypes)]
#[case::keyword_abilities(CatalogResource::KeywordAbilities)]
#[case::keyword_actions(CatalogResource::KeywordActions)]
#[case::land_types(CatalogResource::LandTypes)]
#[case::loyalties(CatalogResource::Loyalties)]
#[case::planeswalker_types(CatalogResource::PlaneswalkerTypes)]
#[case::powers(CatalogResource::Powers)]
#[case::spell_types(CatalogResource::SpellTypes)]
#[case::toughnesses(CatalogResource::Toughnesses)]
#[case::watermarks(CatalogResource::Watermarks)]
#[case::word_bank(CatalogResource::WordBank)]
fn test_blocking_request(response: &String, catalog: &Catalog, #[case]resource: CatalogResource) {
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
        .expect("Expected a valid Catalog response");

    endpoint.assert();
    assert_eq!(catalog, &response)
}

#[rstest]
#[case::ability_words(CatalogResource::AbilityWords)]
#[case::artifact_types(CatalogResource::ArtifactTypes)]
#[case::artist_names(CatalogResource::ArtistNames)]
#[case::card_names(CatalogResource::CardNames)]
#[case::creature_types(CatalogResource::CreatureTypes)]
#[case::enchantment_types(CatalogResource::EnchantmentTypes)]
#[case::keyword_abilities(CatalogResource::KeywordAbilities)]
#[case::keyword_actions(CatalogResource::KeywordActions)]
#[case::land_types(CatalogResource::LandTypes)]
#[case::loyalties(CatalogResource::Loyalties)]
#[case::planeswalker_types(CatalogResource::PlaneswalkerTypes)]
#[case::powers(CatalogResource::Powers)]
#[case::spell_types(CatalogResource::SpellTypes)]
#[case::toughnesses(CatalogResource::Toughnesses)]
#[case::watermarks(CatalogResource::Watermarks)]
#[case::word_bank(CatalogResource::WordBank)]
#[tokio::test]
async fn test_async_request(response: &String, catalog: &Catalog, #[case]resource: CatalogResource) {
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
        .expect("Expected a valid Catalog response");

    endpoint.assert();
    assert_eq!(catalog, &response)
}
