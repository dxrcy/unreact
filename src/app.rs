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
    /// TODO Examples
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
    /// TODO Examples
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
    /// TODO Examples
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
    /// TODO Examples
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
    /// TODO Examples
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
    /// TODO Examples
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
    /// TODO Examples
    pub fn not_found(&mut self, template: &str, data: Object) -> &mut Self {
        self.route("404", template, data)
    }

    /// Compile app to build directory
    ///
    /// Does not open a dev server, even in *dev mode*
    ///
    /// TODO Examples
    pub fn compile(&self) -> Result<(), Error> {
        clean_build_dir(&self.config)?;

        let mut registry = Handlebars::new();

        if self.config.strict {
            registry.set_strict_mode(true);
        }

        register_inbuilt_templates(&mut registry, &self.url)?;

        let templates = read_folder_recurse(&self.config.templates)?;
        register_templates(&mut registry, templates)?;

        for (name, page) in &self.pages {
            let path = format!("./{}/{}", self.config.build, name);

            try_unwrap!(
                fs::create_dir_all(&path),
                else Err(err) => return io_fail!(CreateDir, path, err),
            );

            let content = render_page(
                &mut registry,
                name,
                page,
                self.globals.clone(),
                self.is_dev,
                self.config.minify,
            )?;

            let path = format!("{path}/index.html");
            try_unwrap!(
                fs::write(&path, content),
                else Err(err) => return io_fail!(WriteFile, path, err),
            );
        }

        let styles = read_folder_recurse(&self.config.styles)?;
        for (name, scss) in styles {
            let parent = format!("{}/{}/{}", self.config.build, self.config.styles, name);
            try_unwrap!(
                fs::create_dir_all(&parent),
                else Err(err) => return io_fail!(CreateDir, parent, err),
            );

            let css = scss_to_css(&name, &scss, self.config.minify)?;

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
    /// TODO Examples
    #[cfg(feature = "dev")]
    pub fn run(&self) -> Result<(), Error> {
        if !self.is_dev {
            return self.compile();
        }

        let compile = || {
            try_unwrap!(
                self.compile(),
                else Err(err) => eprintln!("Failed to build in dev mode!\n{:?}", err),
            );
        };

        //TODO Make ~*pretty*~
        println!("Listening on http://localhost:{}", server::SERVER_PORT);

        compile();

        cfg_if!( if #[cfg(feature = "watch")] {
            std::thread::spawn(server::listen);

            server::watch(compile);
        } else {
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
