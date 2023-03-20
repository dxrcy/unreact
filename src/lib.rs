#[macro_use]
mod macros;

mod app;
mod config;
mod convert;
mod files;

#[cfg(feature = "dev")]
mod server;

#[macro_use]
extern crate cfg_if;

use std::collections::HashMap;

pub use serde_json::Value;

pub use config::Config;

pub type Object = serde_json::Map<String, Value>;

type FileMap = HashMap<String, String>;
type Pages = HashMap<String, Page>;
type Result<T = ()> = std::result::Result<T, String>;

pub const DEV_BUILD_DIR: &str = ".devbuild";

#[derive(Debug)]
pub(crate) enum Page {
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
