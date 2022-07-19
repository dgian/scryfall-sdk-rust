use scryfall_sdk_rust::resources::errors::ErrorBody;
use scryfall_sdk_rust::resources::ResourceKind;
use httpmock::Method::GET;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use httpmock::MockServer;
use indoc::indoc;
use rstest::{fixture, rstest};

use scryfall_sdk_rust::{Scryfall, ScryfallBlocking};
use scryfall_sdk_rust::resources::HttpResource;

struct TestEndpoint;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Model {
    pub kind: ResourceKind,
}

impl HttpResource<Model> for TestEndpoint {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        String::from("test-path")
    }
}

#[fixture]
fn error_json() -> String {
    indoc!(r#"
    {
        "object": "error",
        "code": "not_found",
        "status": 404,
        "details": "No card found with the given ID or set code and collector number."
    }
    "#).into()
}

#[fixture]
fn error() -> ErrorBody {
    ErrorBody {
        code: "not_found".into(),
        details: "No card found with the given ID or set code and collector number.".into(),
        error_type: None,
        kind: ResourceKind::Error,
        status: 404,
        warnings: None,
    }
}

#[test]
fn test_blocking_request() {
    let server = MockServer::start();

    let empty_json = r#"{"kind": "card"}"#;

    server.mock(|when, then| {
        when.method(GET).path("/test-path");
        then.body(empty_json);
    });

    let url = server.base_url();
    let client = ScryfallBlocking::from_url(&url);

    let response = client.request(&TestEndpoint).unwrap();
    assert_eq!(Model {kind: ResourceKind::Card}, response)
}

#[tokio::test]
async fn test_async_request() {
    let server = MockServer::start_async().await;

    let empty_json = r#"{"kind": "card"}"#;

    server.mock(|when, then| {
         when.method(GET).path("/test-path");
         then.body(empty_json);
    });

    let url = server.base_url();
    let client = Scryfall::from_url(&url);

    let response = client.request(&TestEndpoint).await.unwrap();
    assert_eq!(Model {kind: ResourceKind::Card}, response)
}

#[rstest]
fn test_blocking_request_responding_error(error_json: String, error: ErrorBody) {
    let server = MockServer::start();

    server.mock(|when, then| {
         when.method(GET).path("/test-path");
         then.body(error_json);
    });

    let url = server.base_url();
    let client = ScryfallBlocking::from_url(&url);

    let error_response = client.request(&TestEndpoint).unwrap_err();
    assert_eq!(error, error_response)
}

#[rstest]
#[tokio::test]
async fn test_async_request_responding_error(error_json: String, error: ErrorBody) {
    let server = MockServer::start_async().await;

    server.mock(|when, then| {
         when.method(GET).path("/test-path");
         then.body(error_json);
    });

    let url = server.base_url();
    let client = Scryfall::from_url(&url);

    let error_response = client.request(&TestEndpoint).await.unwrap_err();
    assert_eq!(error, error_response)
}