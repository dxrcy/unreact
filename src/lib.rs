//! # Unreact
//!
//! [Unreact](https://github.com/darccyy/unreact) is a simple static site generation framework
//!
//! ## Quick Start
//!
//! See also: [examples](https://github.com/darccyy/unreact/tree/main/examples)
//! and [unreact template](https://github.com/darccyy/unreact-template)
//!
//! Create an site with a single index page
//!
//! ```rust,no_run
//! use unreact::prelude::*;
//!
//! fn main() -> Result<(), Error> {
//!    // Create the app
//!    // Using default config, not in dev mode, and an example url
//!    let mut app = Unreact::new(Config::default(), false, "https://example.com")?;
//!    // Create an index route
//!    // This uses the template 'page.hbs' in 'templates/'
//!    // A json object with a value for 'foo' is passed into the template
//!    app.index("page", object! { foo: "World!" });
//!    // Compile it!
//!    app.run()
//! }
//! ```
//!
//! Your workspace should look something like this:
//!
//! ```txt
//! unreact-app/
//!   ├─ Cargo.toml
//!   ├─ src/
//!   │  └─ main.rs
//!   │
//!   └─ assets/
//!      ├─ templates/
//!      │  └─ page.hbs
//!      │
//!      ├─ styles/
//!      └─ public/
//! ```
//!
//! This is the contents of `assets/templates/page.hbs`:
//!
//! ```hbs
//! <h1> Hello {{foo}} </h1>
//! ```
//!
//! This will render `build/index.html`:
//!
//! ```html
//! <h1> Hello World! </h1>
//! ```
//!
//! A larger project could look something like this:
//!
//! ```txt
//! unreact-app/
//!   ├─ Cargo.toml
//!   ├─ src/
//!   │  └─ main.rs
//!   │
//!   └─ assets/
//!      ├─ templates/
//!      │  ├─ boilerplate.hbs
//!      │  ├─ hello.hbs
//!      │  ├─ other/
//!      │  │  ├─ another/
//!      │  │  │  └─ something.hbs
//!      │  │  └─ article.hbs
//!      │  └─ page.hbs
//!      │
//!      ├─ styles/
//!      │  ├─ global.scss
//!      │  └─ scoped/
//!      │     └─ stylish.scss
//!      │
//!      └─ public/
//!         └─ favicon.ico
//! ```

/// Private macros module
#[macro_use]
mod macros;
/// `Unreact` struct implementations
mod app;
/// `Config` struct
mod config;
/// Convert and render filetypes, .hbs and .scss
mod convert;
/// Unreact `Error` type
mod error;
/// Handle file system logic
mod files;

/// Dev server and websockets
#[cfg(feature = "dev")]
mod server;

use handlebars::Handlebars;
use std::collections::HashMap;

pub use crate::{
    config::Config,
    error::{Error, IoError},
};

pub use handlebars;
pub use serde_json::{json, Value};

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

/// Represents json-like object
/// A map of string keys to json values
///
/// A type alias for `serde_json::Map<String, serde_json::Value>`
///
/// See also: [`object`]
pub type Object = serde_json::Map<String, Value>;

/// Map a filepath to file contents
type FileMap = HashMap<String, String>;
/// Map a path to a `Page` enum
type RouteMap = HashMap<String, Page>;

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
    /// Raw string
    Raw(String),
    /// Render a template, with data
    Template { template: String, data: Object },
}

/// Unreact app
///
/// Create a new app with `Unreact::new()`
///
/// # Examples
///
/// ```rust,no_run
/// use unreact::prelude::*;
///
/// const URL: &str = "https://example.com";
///
/// fn main() -> Result<(), Error> {
///    let mut app = Unreact::new(Config::default(), false, URL)?;
///    
///    app
///        .index("page", object! {})?
///        .route("hi", "hello", object! {
///            world: "World!"
///        })?;
///
///    app.run()
/// }
#[derive(Debug)]
pub struct Unreact<'a> {
    /// Configuration for app
    config: Config,
    /// Map paths to pages
    routes: RouteMap,
    /// Global variables for templates
    globals: Object,
    /// Whether *dev mode* is active
    is_dev: bool,
    /// [`Handlebars`](handlebars) registry
    ///
    /// Access with `.handlebars()` method
    handlebars: Handlebars<'a>,
    /// Url of app (overridden in *dev mode*)
    ///
    /// Access with `.url()` method
    url: String,
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

/// Alias for u16
type Port = u16;

/// Local port to host dev server (on localhost)
const DEFAULT_PORT: Port = 3000;
/// Local port to host websocket hub (on localhost)
const DEFAULT_PORT_WS: Port = 3001;

/// Get package name from `Cargo.toml` file in workspace
///
/// Returns `None` if any errors are found, or no package name is found
#[cfg(feature = "dev")]
fn get_package_name() -> Option<String> {
    // Read Cargo.toml or return
    let file = std::fs::read_to_string("./Cargo.toml").ok()?;

    // Current category is 'package'
    let mut is_package = false;
    // Loop lines
    for line in file.lines() {
        let line = line.trim();

        // Change category
        if line.starts_with('[') && line.ends_with(']') {
            is_package = line == "[package]";
        }
        // Continue if not package category
        if !is_package {
            continue;
        }

        // Check key is 'name'
        let mut split = line.split('=');
        if split.next().map(|x| x.trim()) != Some("name") {
            continue;
        }
        let rest: Vec<_> = split.collect();

        // Get rest of line
        let name = rest.join("=");
        let name = name.trim();

        // Remove first and last characters, break if not quotes
        let mut chars = name.chars();
        if chars.next() != Some('"') {
            break;
        }
        if chars.next_back() != Some('"') {
            break;
        }

        // Return value
        return Some(chars.as_str().to_string());
    }

    // No name found
    None
}
