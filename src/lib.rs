///! A static site generation library

#[macro_use]
mod macros;
mod app;
mod config;
mod convert;
mod error;
mod files;

#[cfg(feature = "dev")]
mod server;

#[macro_use]
extern crate cfg_if;

pub use serde_json::Value;

pub use crate::{config::Config, error::Error};

/// Represents json-like object
/// A map of string keys to json values
///
/// A type alias for `serde_json::Map<String, serde_json::Value>`
///
/// See also: [`object`]
pub type Object = serde_json::Map<String, Value>;

use std::collections::HashMap;

/// Map a filepath to file contents
type FileMap = HashMap<String, String>;
/// Map a path to a `Page` enum
type Pages = HashMap<String, Page>;

/// Alias
///
/// TODO Remove
type Result<T = ()> = std::result::Result<T, Error>;

/// Build directory for *dev mode*
///
/// Overrides any build directory given
const DEV_BUILD_DIR: &str = ".devbuild";

/// A page that will be rendered
///
/// ## Variants
///
/// - `Raw`: Raw string
/// - `Template`: Render a template, with data
#[derive(Debug)]
enum Page {
    Raw(String),
    Template { template: String, data: Object },
}

/// Unreact app
///
/// Create a new app with `Unreact::new()`
///
/// TODO Examples
#[derive(Debug)]
pub struct Unreact {
    config: Config,
    pages: Pages,
    globals: Object,
    url: String,
    is_dev: bool,
}

/// Check if `--dev` or `-d` argument was passed on `cargo run`
///
/// # Examples
///
/// This will run in production mode
///
/// ```ps1
/// cargo run
/// ```
///
/// This will run in development mode
///
/// ```ps1
/// cargo run -- --dev
/// cargo run -- -d
/// ```
pub fn is_dev() -> bool {
    let args = std::env::args().collect::<Vec<_>>();
    args.contains(&"--dev".to_string()) || args.contains(&"-d".to_string())
}

/// Prelude for `Unreact`
///
/// ## Contains
///
/// - [`Unreact`] struct
/// - [`Config`] struct
/// - [`object`] macro
/// - [`is_dev`](fn.is_dev.html) function
/// - [`Error`] enum
pub mod prelude {
    pub use crate::{is_dev, object, Config, Error, Unreact};
}
