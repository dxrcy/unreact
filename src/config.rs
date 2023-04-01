/// Configuration struct for `Unreact`
///
/// Use `Config::default()` for default values
///
/// ## Summary
///
/// - `strict`: Whether `Handlebars` uses 'strict mode'
/// - `minify`: Whether output files should be minified
///
/// Folders:
///
/// - `build`: Output folder for built files
/// - `templates`: Source folder for template files
/// - `styles`: Source folder for style files
/// - `public`: Source folder for static public files
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

    /// Whether `Handlebars` uses 'strict mode'
    ///
    /// If `true`, undefined variables and partials throw an error
    pub strict: bool,
    /// Whether output files should be minified
    ///
    /// Only affects `html` and `css` output files
    pub minify: bool,
    //TODO
    // pub dev_logs: bool,
    // pub port_main:
    // pub port_websocket:
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
        }
    }
}
