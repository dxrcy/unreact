use crate::{Port, DEFAULT_PORT, DEFAULT_PORT_WS};

/// Configuration struct for `Unreact`
///
/// Use `Config::default()` for default values
///
/// ## Summary
///
/// - `strict`: Whether [`Handlebars`](handlebars) uses 'strict mode'
/// - `minify`: Whether output files should be minified
///
/// Folders:
///
/// - `build`: Output folder for built files
/// - `templates`: Source folder for template files
/// - `styles`: Source folder for style files
/// - `public`: Source folder for static public files
///
/// Development Options:
///
/// - `port`: Port to serve *dev server* on - Only used with `"dev"` feature
/// - `port_ws`: Port to serve *dev server* **websockets** on - Only used with `"watch"` feature
/// - `watch_logs`: Whether to log update information - Only used with `"watch"` feature
#[derive(Debug)]
pub struct Config {
    /// Output folder for built files
    ///
    /// Overridden with DEV_BUILD_DIR if in dev mode
    ///
    /// Subfolders of build directory cannot be configured
    ///
    /// Default: `build` (or `.devbuild` in dev mode)
    pub build: String,
    /// Source folder for template files
    ///
    /// Default: `templates`
    pub templates: String,
    /// Source folder for style files
    ///
    /// Default: `styles`
    pub styles: String,
    /// Source folder for static public files
    ///
    /// Default: `public`
    pub public: String,

    /// Whether [`Handlebars`](handlebars) uses 'strict mode'
    ///
    /// If `true`, undefined variables and partials throw an error
    pub strict: bool,
    /// Whether output files should be minified
    ///
    /// Only affects `html` and `css` output files
    pub minify: bool,

    /// Port for main *dev server* to be hosted on
    ///
    /// Only used with `"dev"` feature, but must be defined always
    pub port: Port,
    /// Port for websocket server to be hosted on
    ///
    /// Only used with `"watch"` feature, but must be defined always
    pub port_ws: Port,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            build: "build".to_string(),
            templates: "templates".to_string(),
            styles: "styles".to_string(),
            public: "public".to_string(),

            strict: false,
            minify: true,

            port: DEFAULT_PORT,
            port_ws: DEFAULT_PORT_WS,
        }
    }
}
