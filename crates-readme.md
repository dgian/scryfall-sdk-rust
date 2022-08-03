# Scryfall SDK

This is a light SDK (http api-binding) for the amazing [Scryfall search-engine](https://scryfall.com), a powerful search tool for Magic: The Gathering cards, sets etc.

This SDK is implemented mainly for learning and practicing Rust skills.

## Requirements

* **Minimum Rust version**: `1.61`

## Usage

Add dependency to `Cargo.toml`

```toml
[dependencies]
scryfall_sdk_rust = "0.1"
```

The SDK exposes two HTTP clients using [reqwest crate](https://crates.io/crates/reqwest):

- async client (default) via `Scryfall` struct
- blocking client via `ScryfallBlocking` struct

**Note** In order to use the blocking client you have to enable the `blocking` optional feature in Cargo.toml.

```toml
[dependencies]
scryfall_sdk_rust = {version = "0.1", features = ["blocking"] }
```

### Examples

In order to use the SDK, you have to take an instance of either client
and make a request with it using one of the implemented resources.

For example:

Get a single card by Scryfall id (async)
```rust
use std::error::Error;
use scryfall_sdk_rust::{CardResource, Scryfall};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let scryfall = Scryfall::default();

    let card = scryfall.request(
        &CardResource::ById("f295b713-1d6a-43fd-910d-fb35414bf58a")
    ).await?;

    Ok(println!("{:?}", card))
}
```

#### Card search

Scryfall provides a [very powerful search syntax](https://scryfall.com/docs/syntax) which you
can leverage in order to search for cards. **Note** currently the SDK provides only usage of row `q` string in the query parmaeter.
Future versions will provide a Rust fluent api supporting the search keywords separately.

Find red creatures with 3 power (async)
```rust
use std::error::Error;
use scryfall_sdk_rust::{CardPageResource, Scryfall};
use scryfall_sdk_rust::resources::cards::SearchQueryParams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let scryfall = Scryfall::default();

    let search = CardPageResource::Search(
        SearchQueryParams::with_q("c%3Ared+pow%3D3")
    );

    let cards = scryfall.request(&search).await?;

    Ok(println!("{:?}", cards))
}
```
#### Error response handling

In the previous very basic examples, any kind of error,
either an error response from the API, or a client error
is propagated to the `Result` return of `main` function.
This is done through the special `?` rust operator which propagates
errors to the caller of a function.

In reality, you will probably want to handle those errors at some point,
at least the error responses (e.g. 404) from the API. 
The `request` function of both clients returns a `Result<M, ErrorBody>`
which should contain either the Model for the expected object (e.g. Card) in case of success, or an `ErrorBody` in case of an error. For more info on the error response payloads (ErrorBody) see [Scryfall documentation](https://scryfall.com/docs/api/errors).

An example of a possible error handling is the following

```rust
use std::error::Error;
use scryfall_sdk_rust::{CardResource, Scryfall};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let scryfall = Scryfall::default();

    let card = scryfall.request(
        &CardResource::ById("f295b713-1d6a-43fd-910d-fb35414bf58a")
    ).await; // <1>

    println!("{:?}", card
        .map_err(|e| format!("Error {}: {}", e.status, e.details)
    )); // <2>

    let error = scryfall.request(
        &CardResource::ById("invalid")
    ).await; // <3>

    Ok(println!("{:?}", error
        .map_err(|e| format!("Error {}: {}", e.status, e.details))
    )) // <4>
}
```

For client errors, e.g. when the Scryfall API server cannot be resolved,
or when the json response cannot be decoded for some reason,
a special `ErrorBody` will be returned. This will have `code = CLIENT_ERR`
and `status = 599` with `details` containing the original error cause.

## List of implemented resources

The following are currently implemented:

- `CardResource` -> https://scryfall.com/docs/api/cards (single)
- `CardPageResource` -> https://scryfall.com/docs/api/cards (page/search)
- `CardCatalogResource` -> https://scryfall.com/docs/api/cards/autocomplete
- `CardCollectionResource` -> https://scryfall.com/docs/api/cards/collection
- `BulkDataListResource` -> https://scryfall.com/docs/api/bulk-data (list)
- `BulkDataResource` -> https://scryfall.com/docs/api/bulk-data (single)
- `CatalogResource` -> https://scryfall.com/docs/api/catalogs
- `CardSymbolsResource` -> https://scryfall.com/docs/api/card-symbols/all
- `ManaCostResource` -> https://scryfall.com/docs/api/card-symbols/parse-mana
- `CardSetListResource` -> https://scryfall.com/docs/api/sets (list)
- `CardSetResource` -> https://scryfall.com/docs/api/sets (single)
- `RulingListResource` -> https://scryfall.com/docs/api/rulings
