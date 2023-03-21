use std::fs;

use handlebars::Handlebars;

#[cfg(feature = "dev")]
use crate::server;
use crate::{
    convert::{register_inbuilt_templates, register_templates, render_page, scss_to_css},
    files::{check_source_folders, clean_build_dir, read_folder_recurse},
    object, Config, Error, Object, Page, Pages, Unreact, DEV_BUILD_DIR,
};

impl Unreact {
    /// Create a new empty `Unreact` app
    ///
    /// ## Parameters
    ///
    /// - `config`: Configuration for the app (See [`Config`])
    /// - `is_dev`: Whether the app should build in *dev mode* (See [`is_dev`](fn.is_dev.html))
    /// - `url`: The url that should be given to rendered templates. Overridden in *dev mode*
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
    ///         # build: ".devbuild/a".to_string(),
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
    ///         .route_bare("article", "other/article");
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

        Ok(Self {
            config,
            pages: Pages::new(),
            globals: Object::new(),
            url: get_url(url, is_dev),
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

    /// Create a route
    ///
    /// ## Parameters
    ///
    /// - `path`: The folder (relative to build directory) that file should be written in (`{build}/{path}/index.html`)
    /// - `template`: The name of the template to use
    /// - `data`: Data to pass into the template, as an `Object`
    ///
    /// # Examples
    ///
    /// ```,no_run
    /// # use unreact::prelude::*;
    /// # fn main() -> Result<(), Error> {
    ///     # let mut app = Unreact::new(Config::default(), false, "https://example.com")?;
    /// app
    ///     // Create a route to '/some_path' with the template 'page.hbs' and a message
    ///     .route("some_path", "page", object! {message: "this is at '/some_path'"})
    ///     // Create a route without a template (raw string)
    ///     .route_raw("hello", "this is my hello page".to_string())
    ///     // Create a route without data
    ///     .route_bare("article", "other/article")
    ///     // Index page with a message
    ///     .index("page", object! {message: "World"})
    ///     // 404 page with no data
    ///     .not_found("404", object! {});
    /// # app.compile()
    /// # }
    /// ```
    ///
    /// # See also
    ///
    /// - [`route_raw`](struct.Unreact.html#method.route_raw): Create a route without a template
    /// - [`route_bare`](struct.Unreact.html#method.route_bare): Create a route without any data
    /// - [`index`](struct.Unreact.html#method.index): Create an index route (`/`)
    /// - [`not_found`](struct.Unreact.html#method.not_found): Create a 404 route (`/404`)
    ///
    pub fn route(&mut self, path: &str, template: &str, data: Object) -> &mut Self {
        self.pages.insert(
            path.to_string(),
            Page::Template {
                template: template.to_string(),
                data,
            },
        );
        self
    }

    /// Create a route, with raw page content instead of a template
    ///
    /// ## Parameters
    ///
    /// - `path`: The folder (relative to build directory) that file should be written in (`{build}/{path}/index.html`)
    /// - `content`: The raw file contents to write to the file
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use unreact::prelude::*;
    /// # fn main() -> Result<(), Error> {
    ///     # let mut app = Unreact::new(Config::default(), false, "https://example.com")?;
    /// app
    ///     // Create a route to '/some_path' with the template 'page.hbs' and a message
    ///     .route("some_path", "page", object! {message: "this is at '/some_path'"})
    ///     // Create a route without a template (raw string)
    ///     .route_raw("hello", "this is my hello page".to_string())
    ///     // Create a route without data
    ///     .route_bare("article", "other/article")
    ///     // Index page with a message
    ///     .index("page", object! {message: "World"})
    ///     // 404 page with no data
    ///     .not_found("404", object! {});
    /// # app.compile()
    /// # }
    /// ```
    ///
    /// # See also
    ///
    /// - [`route`](struct.Unreact.html#method.route): Create a normal route
    /// - [`route_bare`](struct.Unreact.html#method.route_bare): Create a route without any data
    /// - [`index`](struct.Unreact.html#method.index): Create an index route (`/`)
    /// - [`not_found`](struct.Unreact.html#method.not_found): Create a 404 route (`/404`)
    ///
    pub fn route_raw(&mut self, path: &str, content: String) -> &mut Self {
        self.pages.insert(path.to_string(), Page::Raw(content));
        self
    }

    /// Create a route without any data given to the template
    ///
    /// ## Parameters
    ///
    /// - `path`: The folder (relative to build directory) that file should be written in (`{build}/{path}/index.html`)
    /// - `template`: The name of the template to use
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use unreact::prelude::*;
    /// # fn main() -> Result<(), Error> {
    ///     # let mut app = Unreact::new(Config::default(), false, "https://example.com")?;
    /// app
    ///     // Create a route to '/some_path' with the template 'page.hbs' and a message
    ///     .route("some_path", "page", object! {message: "this is at '/some_path'"})
    ///     // Create a route without a template (raw string)
    ///     .route_raw("hello", "this is my hello page".to_string())
    ///     // Create a route without data
    ///     .route_bare("article", "other/article")
    ///     // Index page with a message
    ///     .index("page", object! {message: "World"})
    ///     // 404 page with no data
    ///     .not_found("404", object! {});
    /// # app.compile()
    /// # }
    /// ```
    ///
    /// # See also
    ///
    /// - [`route`](struct.Unreact.html#method.route): Create a normal route
    /// - [`route_raw`](struct.Unreact.html#method.route_raw): Create a route without a template
    /// - [`index`](struct.Unreact.html#method.index): Create an index route (`/`)
    /// - [`not_found`](struct.Unreact.html#method.not_found): Create a 404 route (`/404`)
    ///
    pub fn route_bare(&mut self, path: &str, template: &str) -> &mut Self {
        self.route(path, template, object! {})
    }

    /// Create the index route
    ///
    /// Alias of `app.route("", ...)`
    ///
    /// File is written to `{build}/index.html`
    ///
    /// ## Parameters
    ///
    /// - `template`: The name of the template to use
    /// - `data`: Data to pass into the template, as an `Object`
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use unreact::prelude::*;
    /// # fn main() -> Result<(), Error> {
    ///     # let mut app = Unreact::new(Config::default(), false, "https://example.com")?;
    /// app
    ///     // Create a route to '/some_path' with the template 'page.hbs' and a message
    ///     .route("some_path", "page", object! {message: "this is at '/some_path'"})
    ///     // Create a route without a template (raw string)
    ///     .route_raw("hello", "this is my hello page".to_string())
    ///     // Create a route without data
    ///     .route_bare("article", "other/article")
    ///     // Index page with a message
    ///     .index("page", object! {message: "World"})
    ///     // 404 page with no data
    ///     .not_found("404", object! {});
    /// # app.compile()
    /// # }
    /// ```
    ///
    /// # See also
    ///
    /// - [`route`](struct.Unreact.html#method.route): Create a normal route
    /// - [`route_raw`](struct.Unreact.html#method.route_raw): Create a route without a template
    /// - [`route_bare`](struct.Unreact.html#method.route_bare): Create a route without any data
    /// - [`not_found`](struct.Unreact.html#method.not_found): Create a 404 route (`/404`)
    ///
    pub fn index(&mut self, template: &str, data: Object) -> &mut Self {
        self.route("", template, data)
    }

    /// Create the 404 route
    ///
    /// Alias of `app.route("404", ...)`.
    /// Used as the 404 page, for a path not found
    ///
    /// File is written to `{build}/404/index.html`
    ///
    /// ## Parameters
    ///
    /// - `template`: The name of the template to use
    /// - `data`: Data to pass into the template, as an `Object`
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use unreact::prelude::*;
    /// # fn main() -> Result<(), Error> {
    ///     # let mut app = Unreact::new(Config::default(), false, "https://example.com")?;
    /// app
    ///     // Create a route to '/some_path' with the template 'page.hbs' and a message
    ///     .route("some_path", "page", object! {message: "this is at '/some_path'"})
    ///     // Create a route without a template (raw string)
    ///     .route_raw("hello", "this is my hello page".to_string())
    ///     // Create a route without data
    ///     .route_bare("article", "other/article")
    ///     // Index page with a message
    ///     .index("page", object! {message: "World"})
    ///     // 404 page with no data
    ///     .not_found("404", object! {});
    /// # app.compile()
    /// # }
    /// ```
    ///
    /// # See also
    ///
    /// - [`route`](struct.Unreact.html#method.route): Create a normal route
    /// - [`route_raw`](struct.Unreact.html#method.route_raw): Create a route without a template
    /// - [`route_bare`](struct.Unreact.html#method.route_bare): Create a route without any data
    /// - [`index`](struct.Unreact.html#method.index): Create an index route (`/`)
    ///
    pub fn not_found(&mut self, template: &str, data: Object) -> &mut Self {
        self.route("404", template, data)
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
        register_inbuilt_templates(&mut registry, &self.url)?;

        // Register custom templates
        let templates = read_folder_recurse(&self.config.templates)?;
        register_templates(&mut registry, templates)?;

        // Render page and write to files
        for (name, page) in &self.pages {
            // Create folder for `index.html` file
            let parent = format!("{}/{}", self.config.build, name);
            try_unwrap!(
                fs::create_dir_all(&parent),
                else Err(err) => return io_fail!(CreateDir, parent, err),
            );

            // Render page with data
            let content = render_page(
                &mut registry,
                name,
                page,
                self.globals.clone(),
                self.is_dev,
                self.config.minify,
            )?;

            // Write file
            let path = format!("{parent}/index.html");
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
    pub fn run(&self) -> Result {
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
        // Just compile if not dev mode
        if !self.is_dev {
            return self.compile();
        }

        // Create callback with non-breaking error message
        let compile = || {
            try_unwrap!(
                self.compile(),
                else Err(err) => eprintln!("Failed to build in dev mode!\n{:?}", err),
            );
        };

        // Compile for first time
        compile();

        // If watch feature is enabled
        cfg_if!( if #[cfg(feature = "watch")] {
            println!("Listening on http://localhost:{}\nWatching files for changes...", server::SERVER_PORT);

            // Open server in new thread
            std::thread::spawn(server::listen);
            // Watch files for changes
            server::watch(compile);
        } else {
            // Open server in current thread
            println!("Listening on http://localhost:{}", server::SERVER_PORT);

            server::listen();
        });

        Ok(())
    }
}

/// Get the url for the site
///
/// Returns url given, unless `"dev"` feature is enabled and *dev mode* is active
fn get_url(url: &str, #[allow(unused_variables)] is_dev: bool) -> String {
    // If `watch` feature is used, and `is_dev`
    cfg_if!( if #[cfg(feature = "dev")] {
        if is_dev {
            return format!("http://localhost:{}", server::SERVER_PORT);
        }
    });

    // Default
    url.to_string()
}
