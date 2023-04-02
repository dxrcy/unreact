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
//! ```
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
//!    app.compile()
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
//!   ├─ templates/
//!   │  └─ page.hbs
//!   │
//!   ├─ styles/
//!   └─ public/
//! ```
//!
//! This is the contents of `templates/page.hbs`:
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
//!   ├─ templates/
//!   │  ├─ boilerplate.hbs
//!   │  ├─ hello.hbs
//!   │  ├─ other/
//!   │  │  ├─ another/
//!   │  │  │  └─ something.hbs
//!   │  │  └─ article.hbs
//!   │  └─ page.hbs
//!   │
//!   ├─ styles/
//!   │  ├─ global.scss
//!   │  └─ scoped/
//!   │     └─ stylish.scss
//!   │
//!   └─ public/
//!      └─ favicon.ico
//! ```

#[macro_use]
mod macros;
mod app;
mod config;
mod convert;
mod error;
mod files;

#[cfg(feature = "dev")]
mod server;

pub use serde_json::Value;

pub use crate::{
    config::Config,
    error::{Error, IoError},
};

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
    Raw(String),
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
///        .index("page", object! {})
///        .route("hi", "hello", object! {
///            world: "World!"
///        });
///
///    app.run()
/// }
#[derive(Debug)]
pub struct Unreact {
    config: Config,
    routes: RouteMap,
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
