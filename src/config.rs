#[derive(Debug)]
pub struct Config {
    pub build: String,
    pub templates: String,
    pub styles: String,
    pub public: String,

    pub strict: bool,
    pub minify: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            build: "build".to_string(),
            templates: "templates".to_string(),
            public: "public".to_string(),
            styles: "styles".to_string(),
            strict: false,
            minify: true,
        }
    }
}
