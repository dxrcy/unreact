mod routes;

use std::fs;

use handlebars::Handlebars;

use crate::{
    convert::{register_partials, register_templates, render_page, scss_to_css},
    files::{check_source_folders, clean_build_dir, read_folder_recurse},
    Config, Error, Object, Port, RouteMap, Unreact, DEV_BUILD_DIR,
};

impl Unreact {
    /// Create a new empty `Unreact` app
    ///
    /// ## Parameters
    ///
    /// - `config`: Configuration for the app (See [`Config`])
    /// - `is_dev`: Whether the app should build in *dev mode* (See [`is_dev`](fn.is_dev.html))
    /// - `url`: The url that should be given to rendered templates. Overridden in *dev mode*. Trailing forward-slash is added if not present
    ///
    /// # Examples
    ///
    /// ### Quick Start
    ///
    /// ```rust,no_run
    /// use unreact::prelude::*;
    ///
    /// fn main() -> Result<(), Error> {
    ///     // Create the app
    ///     // Using default config, not in dev mode, and an example url
    ///     let mut app = Unreact::new(Config::default(), false, "https://example.com")?;
    ///     // Create an index route
    ///     // This uses the template 'page.hbs' in 'templates/'
    ///     // A json object with a value for 'foo' is passed into the template
    ///     app.index("page", object! { foo: "World!" });
    ///     // Compile it!
    ///     app.compile()
    /// }
    /// ```
    ///
    /// ### Long Example
    ///
    /// ```rust,no_run
    /// use unreact::prelude::*;
    ///
    /// fn main() -> Result<(), Error> {
    ///     // Custom config
    ///     let config = Config {
    ///         // Strict mode enabled
    ///         strict: true,
    ///         ..Config::default()
    ///     };
    ///
    ///     // Create app, with `is_dev` function
    ///     let mut app = Unreact::new(config, is_dev(), "https://bruh.news/").expect("Could not create app");
    ///
    ///     // Set a global variable named 'smiley'
    ///     app.globalize(object! {
    ///         smiley: "(^_^)"
    ///     });
    ///
    ///     // Some routes
    ///     app.index("page", object! {message: "World"})
    ///         .not_found("404", object! {})
    ///         .route_raw("hello", "this is my hello page".to_string())
    ///         .route("article", "other/article", object! {});
    ///     
    ///     // Run app
    ///     app.run().expect("Could not compile app");
    ///
    ///     println!("Compiled successfully");
    ///     Ok(())
    /// }
    /// ```
    pub fn new(mut config: Config, is_dev: bool, url: &str) -> Result<Self, Error> {
        // Use dev build directory if dev mode is active
        if is_dev {
            config.build = DEV_BUILD_DIR.to_string();
        }

        // Check that source folders exist and can be accessed
        check_source_folders(&config)?;

        // Override url if in dev mode
        let url = get_url(url, is_dev, config.port);

        Ok(Self {
            config,
            routes: RouteMap::new(),
            globals: Object::new(),
            url,
            is_dev,
        })
    }

    /// Set global variables for templates
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use unreact::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// Unreact::new(Config::default(), false, "https://example.com")?
    ///     // Index page
    ///     .index("page", object! {})
    ///     // Globalize does not need to be ran before routes
    ///     .globalize(object! {smiley: "(^_^)"})
    ///     // Compiles with a smiley face replacing `{{GLOBAL.smiley}}`
    ///     .compile()
    /// # }
    /// ```
    pub fn globalize(&mut self, data: Object) -> &mut Self {
        self.globals = data;
        self
    }

    /// Compile app to build directory
    ///
    /// Does not open a dev server, even in *dev mode*
    ///
    /// # Examples
    ///
    /// ### Quick Start
    ///
    /// ```rust,no_run
    /// use unreact::prelude::*;
    ///
    /// fn main() -> Result<(), Error> {
    ///     // Create the app
    ///     // Using default config, not in dev mode, and an example url
    ///     let mut app = Unreact::new(Config::default(), false, "https://example.com")?;
    ///     // Create an index route
    ///     // This uses the template 'page.hbs' in 'templates/'
    ///     // A json object with a value for 'foo' is passed into the template
    ///     app.index("page", object! { foo: "World!" });
    ///     // Compile it!
    ///     app.compile()
    /// }
    /// ```
    pub fn compile(&self) -> Result<(), Error> {
        clean_build_dir(&self.config)?;

        // Create handlebars registry
        let mut registry = Handlebars::new();

        // Enable strict mode if active
        if self.config.strict {
            registry.set_strict_mode(true);
        }

        // Register inbuilt templates
        //TODO Check duplicates
        register_partials(&mut registry, &self.url)?;

        // Register custom templates
        let templates = read_folder_recurse(&self.config.templates)?;
        register_templates(&mut registry, templates)?;

        // Render page and write to files
        for (name, page) in &self.routes {
            // Render page with data
            let content = render_page(
                &mut registry,
                page,
                self.globals.clone(),
                self.config.minify,
                self.is_dev,
                self.config.port_ws,
            )?;

            // Get filepath
            let path = if name == "404" {
                // Special case for 404 route
                format!("{}/404.html", self.config.build)
            } else {
                // Create folder for `index.html` file
                let parent = format!("{}/{}", self.config.build, name);
                try_unwrap!(
                    fs::create_dir_all(&parent),
                    else Err(err) => return io_fail!(CreateDir, parent, err),
                );
                // Normal path
                format!("{parent}/index.html")
            };

            // Write file
            try_unwrap!(
                fs::write(&path, content),
                else Err(err) => return io_fail!(WriteFile, path, err),
            );
        }

        // Convert scss to css and write to files
        let styles = read_folder_recurse(&self.config.styles)?;
        for (name, scss) in styles {
            // Create folder for `style.css` file
            let parent = format!("{}/{}/{}", self.config.build, self.config.styles, name);
            try_unwrap!(
                fs::create_dir_all(&parent),
                else Err(err) => return io_fail!(CreateDir, parent, err),
            );

            // Convert to scss
            let css = scss_to_css(&name, &scss, self.config.minify)?;

            // Write file
            let path = format!("{}/style.css", parent);
            try_unwrap!(
                fs::write(&path, css),
                else Err(err) => return io_fail!(WriteFile, path, err),
            );
        }

        Ok(())
    }

    /// Compile app to build directory
    ///
    /// Alias for `app.compile`, without the `"dev"` feature enabled
    ///
    /// Add `features = "dev"` or `features = "watch"` to the `unreact` dependency in `Cargo.toml` to use the 'dev server'
    #[cfg(not(feature = "dev"))]
    pub fn run(&self) -> Result<(), Error> {
        self.compile()
    }

    /// Compile app to build directly, and open local server if *dev mode* is active
    ///
    /// Only opens a dev server with the `"dev"` or `"watch"` features enabled
    ///
    /// If the `"watch"` feature is enabled, source files will also be watched for changes, and the client will be reloaded automatically
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use unreact::prelude::*;
    ///
    /// fn main() -> Result<(), Error> {
    ///     // Create the app
    ///     // Using default config, not in dev mode, and an example url
    ///     let mut app = Unreact::new(Config::default(), true, "https://example.com")?;
    ///     // Create an index route
    ///     // This uses the template 'page.hbs' in 'templates/'
    ///     // A json object with a value for 'foo' is passed into the template
    ///     app.index("page", object! { foo: "World!" });
    ///     // Compile it!
    ///     app.run()
    /// }
    /// ```
    #[cfg(feature = "dev")]
    pub fn run(&self) -> Result<(), Error> {
        use crate::server;
        use stilo::{eprintln_styles, print_styles, println_styles};

        // Just compile if not dev mode
        if !self.is_dev {
            return self.compile();
        }

        // Create callback with non-breaking error message
        let compile = || {
            try_unwrap!(
                self.compile(),
                // Error message
                else Err(err) => eprintln_styles!(
                    "───────────────────────────": Cyan;
                    "\n";
                    "Error compiling in dev mode": Red + bold;
                    "\n";
                    "{}": Yellow, err;
                    "\n";
                    "───────────────────────────": Cyan;
                )
            );
        };

        // Print message before compile
        print_styles!(
            "\nUnreact": Blue + bold + italic;
            " dev server": Blue + bold;
        );
        if let Some(name) = crate::get_package_name() {
            println_styles!(
                " | ": Blue + dim;
                "{}": Magenta, name;
            );
        } else {
            println!();
        }
        println_styles!(
            "Listening on ": Green + bold;
            "http://localhost:{}": Green + bold + underline, self.config.port;
        );
        #[cfg(feature = "watch")]
        {
            println_styles!(
                "    Rust code won't update without 'cargo run'": Yellow;
                "\n    Watching files for changes...": Cyan;
            );
        }
        #[cfg(not(feature = "watch"))]
        {
            println_styles!(
                "    Note: ": Cyan + dim + bold;
                "\"watch\"": Cyan + dim  + italic;
                " feature not enabled": Cyan + dim;
            );
        }
        println!();

        // Compile for first time
        compile();

        // For "watch" feature
        #[cfg(feature = "watch")]
        {
            // Open server in new thread
            let port = self.config.port;
            let port_ws = self.config.port_ws;
            std::thread::spawn(move || server::listen(port, port_ws));

            // Watch files for changes
            server::watch(compile, self.config.port_ws, self.config.watch_logs);
        }

        // For NOT "watch" feature
        #[cfg(not(feature = "watch"))]
        {
            // Open server in current thread
            server::listen(self.config.port, self.config.port_ws);
        }

        Ok(())
    }
}

/// Get the url for the site
///
/// Returns url given, unless `"dev"` feature is enabled and *dev mode* is active
fn get_url(
    url: &str,
    // Only for "dev" feature
    #[allow(unused_variables)] is_dev: bool,
    #[allow(unused_variables)] port: Port,
) -> String {
    // If `watch` feature is used, and `is_dev`
    #[cfg(feature = "dev")]
    {
        if is_dev {
            return format!("http://localhost:{}/", port);
        }
    }

    // Default (add slash to end if not included)
    url.to_string() + if url.ends_with('/') { "" } else { "/" }
}
