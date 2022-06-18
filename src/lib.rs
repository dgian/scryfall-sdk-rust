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

pub mod client;
pub mod resources;

#[doc(inline)]
pub use resources::bulk_data;
#[doc(inline)]
pub use client::Scryfall;
#[doc(inline)]
#[cfg(feature = "blocking")]
pub use client::blocking::Scryfall as ScryfallBlocking;
#[doc(inline)]
pub use resources::HttpResource;
#[doc(inline)]
pub use resources::bulk_data::BulkDataResource;