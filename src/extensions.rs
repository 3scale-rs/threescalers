//! Extensions for 3scale API calls.
//!
//! Extensions allow you to request additional data or modify the behavior of API calls
//! to the 3scale Service Management API. Common extensions include:
//! - `list_app_keys`: Request the list of application keys for an app
//! - `hierarchy`: Request the metrics hierarchy in the response
//! - `flat_usage`: Use a flat structure for usage data instead of nested
//! - `no_body`: Request that the response body be omitted

mod extension;
mod list;

pub use extension::*;
pub use list::*;
