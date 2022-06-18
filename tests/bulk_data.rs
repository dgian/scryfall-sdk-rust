use chrono::{DateTime, Utc};
use httpmock::MockServer;
use httpmock::Method::GET;
use indoc::indoc;
use rstest::{fixture, rstest};
use url::Url;

use scryfall_sdk_rust::{Scryfall, ScryfallBlocking};
use scryfall_sdk_rust::resources::bulk_data::{BulkData, BulkDataEntry, BulkDataResource, EntryKind};
use scryfall_sdk_rust::resources::ResourceKind;

#[fixture]
#[once]
fn response() -> String {
    indoc!(r#"
    {
      "object": "list",
      "has_more": false,
      "data": [
        {
          "object": "bulk_data",
          "id": "27bf3214-1271-490b-bdfe-c0be6c23d02e",
          "type": "oracle_cards",
          "updated_at": "2022-06-18T09:02:10.928+00:00",
          "uri": "https://some-url.com",
          "name": "Oracle Cards",
          "description": "A description",
          "compressed_size": 13976935,
          "download_uri": "https://some-url.com",
          "content_type": "application/json",
          "content_encoding": "gzip"
        }
      ]
    }
    "#).into()
}

#[fixture]
#[once]
fn bulk_data() -> BulkData {
    BulkData {
        kind: ResourceKind::List,
        has_more: false,
        data: vec![BulkDataEntry {
            item_kind: ResourceKind::BulkData,
            id: "27bf3214-1271-490b-bdfe-c0be6c23d02e".into(),
            kind: EntryKind::OracleCards,
            updated_at: "2022-06-18T09:02:10.928+00:00".parse::<DateTime<Utc>>().unwrap(),
            uri: "https://some-url.com".parse::<Url>().unwrap(),
            name: "Oracle Cards".into(),
            description: "A description".into(),
            compressed_size: 13976935,
            download_uri: "https://some-url.com".parse::<Url>().unwrap(),
            content_type: "application/json".into(),
            content_encoding: "gzip".into()
        }]
    }
}

// -- BulkDataResource::All tests
mod all {
    use super::*;

    #[rstest]
    fn test_blocking_get_bulk_data(response: &String, bulk_data: &BulkData) {
        let server = MockServer::start();

            let endpoint = server.mock(|when, then| {
                when.method(GET).path("/bulk-data");
                then.status(200)
                    .header("content-type", "application/json")
                    .body(response);
            });

        let url = server.base_url();
        let client = ScryfallBlocking::from_url(&url);

        let response = client
            .request(&BulkDataResource::All)
            .expect("Expected a valid BulkData response");

        endpoint.assert();
        assert_eq!(bulk_data, &response)
    }

    #[rstest]
    #[tokio::test]
    async fn test_async_request(response: &String, bulk_data: &BulkData) {
        let server = MockServer::start_async().await;

        let endpoint = server.mock(|when, then| {
            when.method(GET).path("/bulk-data");
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });

        let url = server.base_url();
        let client = Scryfall::from_url(&url);

        let response = client
            .request(&BulkDataResource::All).await
            .expect("Expected a valid BulkData response");

        endpoint.assert();
        assert_eq!(bulk_data, &response)
    }
}

// -- BulkDataResource::Filter tests
mod filter {
    use super::*;

    #[rstest]
    fn test_blocking_request(response: &String, bulk_data: &BulkData) {
        let server = MockServer::start();

            let endpoint = server.mock(|when, then| {
                when.method(GET).path("/bulk-data/id");
                then.status(200)
                    .header("content-type", "application/json")
                    .body(response);
            });

        let url = server.base_url();
        let client = ScryfallBlocking::from_url(&url);

        let response = client
            .request(&BulkDataResource::Filter("id"))
            .expect("Expected a valid BulkData response");

        endpoint.assert();
        assert_eq!(bulk_data, &response)
    }

    #[rstest]
    #[tokio::test]
    async fn test_async_request(response: &String, bulk_data: &BulkData) {
        let server = MockServer::start_async().await;

        let endpoint = server.mock(|when, then| {
            when.method(GET).path("/bulk-data/id");
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });

        let url = server.base_url();
        let client = Scryfall::from_url(&url);

        let response = client
            .request(&BulkDataResource::Filter("id")).await
            .expect("Expected a valid BulkData response");

        endpoint.assert();
        assert_eq!(bulk_data, &response)
    }
}