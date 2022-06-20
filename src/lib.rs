//! A simple SDK for [Scryfall API](https://scryfall.com/docs/api)
//!
//! Exposes two clients for async and blocking communication with the API
//! using [reqwest](reqwest) as the underlying HTTP client.
//!
//! - [Scryfall](Scryfall)
//! - [ScryfallBlocking](ScryfallBlocking)
//!
//! Bindings for the following resources are implemented:
//!
//! - <https://api.scryfall.com/bulk-data>
//! - <https://api.scryfall.com/catalog>

pub mod client;
pub mod resources;

// -- Clients
#[doc(inline)]
pub use client::Scryfall;
#[doc(inline)]
#[cfg(feature = "blocking")]
pub use client::blocking::Scryfall as ScryfallBlocking;

// -- Resources
#[doc(inline)]
pub use resources::HttpResource;
#[doc(inline)]
pub use resources::bulk_data::BulkDataResource;
#[doc(inline)]
pub use resources::catalog::CatalogResource;
