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
pub type Object = serde_json::Map<String, Value>;

pub const DEV_BUILD_DIR: &str = ".devbuild";

use std::collections::HashMap;

type FileMap = HashMap<String, String>;
type Pages = HashMap<String, Page>;

type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Page {
    Raw(String),
    Template { template: String, data: Object },
}

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

pub mod prelude {
    pub use crate::{is_dev, object, Config, Error, Unreact};
}
