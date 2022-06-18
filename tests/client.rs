use httpmock::Method::GET;
use reqwest::Method;
use serde_json::Value;
use httpmock::MockServer;

use scryfall_sdk_rust::{Scryfall, ScryfallBlocking};
use scryfall_sdk_rust::resources::HttpResource;

struct TestEndpoint;

impl HttpResource<Value> for TestEndpoint {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        String::from("test-path")
    }
}

#[test]
fn test_blocking_request() {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(GET).path("/test-path");
        then.json_body(r#"{"is_empty": true}"#);
    });

    let url = server.base_url();
    let client = ScryfallBlocking::from_url(&url);

    let response = client.request(&TestEndpoint).unwrap();
    assert_eq!(r#"{"is_empty": true}"#, response)
}

#[tokio::test]
async fn test_async_request() {
    let server = MockServer::start_async().await;

    let empty_json = "{}";

    server.mock(|when, then| {
         when.method(GET).path("/test-path");
         then.json_body(empty_json);
    });

    let url = server.base_url();
    let client = Scryfall::from_url(&url);

    let response = client.request(&TestEndpoint).await.unwrap();
    assert_eq!(empty_json, response)
}
