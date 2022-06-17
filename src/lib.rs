//! A simple SDK for [Scryfall API](https://scryfall.com/docs/api)
//!
//! Exposes two clients for async and blocking communication with the API
//! using [reqwest](reqwest) as the underlying HTTP client.
//!
//! - [Scryfall](Scryfall)
//! - [ScryfallBlocking](ScryfallBlocking)
//!
//! The following endpoints are implemented:
//!
//! *WIP*

pub mod client;

#[doc(inline)]
pub use client::Scryfall;
#[doc(inline)]
pub use client::Endpoint;
#[doc(inline)]
pub use client::blocking::Scryfall as ScryfallBlocking;
