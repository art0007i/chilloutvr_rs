//! Typed model predictions for [`ChilloutVR`](https://store.steampowered.com/app/661130/ChilloutVR/)'s upcoming API.
//!
//! As of writing this, the API shouldn't be used yet by third party clients.
//! The purpose of this library is to have models to base client applications on
//! before the API gets more stable & public.
//!
//! Please contact someone from the ABI team & ask for their permission before
//! using the API, as it seems to be internal only as of writing this and very
//! "don't touch". I've not tried what happens if you do contact the api, so
//! don't complaining if you get punished for touching it.

#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(clippy::cargo)]
//#![warn(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// Strum macros would cause warnings
#![allow(clippy::use_self)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]

//pub const API_V1_HTTP_URL: &str = "https://api.abinteractive.net/v1";
//pub const API_V1_WS_URL: &str = "https://api.abinteractive.net/v1/users/ws";

pub mod ws;

mod requests;
pub use requests::*;

mod users;
pub use users::*;

mod worlds;
pub use worlds::*;

mod instances;
pub use instances::*;

mod featureds;
pub use featureds::*;

mod assets;
pub use assets::*;

#[cfg(feature = "api_client")]
#[cfg_attr(nightly, doc(cfg(feature = "api_client")))]
pub mod api_client;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response<T> {
	pub message: Option<String>,
	pub data: Option<T>,
}
