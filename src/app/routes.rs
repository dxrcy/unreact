use std::path::Path;

use crate::{Error, Object, Page, Unreact};

/// Append shared documentation attributes to each function
macro_rules! include_shared_docs {
    (
        $(
            $( #[$attr:meta] )+
            <::>
            $item:item
        )*
    ) => {
        $(
            $( #[$attr] )+
            /// # Examples
            ///
            /// ```,no_run
            /// # use unreact::prelude::*;
            /// # fn main() -> Result<(), Error> {
            ///     # let mut app = Unreact::new(Config::default(), false, "https://example.com")?;
            /// app
            ///     // Create a route to '/some_path' with the template 'page.hbs' and a message
            ///     .route("some_path", "page", object! {message: "this is at '/some_path'"})?
            ///     // Create a route without a template (raw string)
            ///     .route_raw("hello", "this is my hello page".to_string())
            ///     // Create a route without data
            ///     .route("article", "other/article", object! {})?
            ///     // Index page with a message
            ///     .index("page", object! {message: "World"})?
            ///     // 404 page with no data
            ///     .not_found("404", object! {})?;
            /// # app.run()
            /// # }
            /// ```
            ///
            /// # Routing Methods
            ///
            /// - [`route`](struct.Unreact.html#method.route): Create a normal route
            /// - [`route_raw`](struct.Unreact.html#method.route_raw): Create a route without a template
            /// - [`route_raw_html`](struct.Unreact.html#method.route_raw_html): Create a HTML page route without a template
            /// - [`index`](struct.Unreact.html#method.index): Create an index route (`/`)
            /// - [`not_found`](struct.Unreact.html#method.not_found): Create a 404 route (`/404`)
            $item
        )*
    };
}

impl<'a> Unreact<'a> {
    include_shared_docs!(
        /// Create a route
        ///
        /// **NOTE**: Route will only validate if template exists in production. In dev mode, this function **will always pass**, and error will occur during `run` function
        ///
        /// ## Parameters
        ///
        /// - `path`: The folder (relative to build directory) that file should be written in (`{build}/{path}/index.html`)
        /// - `template`: The name of the template to use
        /// - `data`: Data to pass into the template, as an `Object`
        <::>
        pub fn route(&mut self, path: &str, template: &str, data: Object) -> Result<&mut Self, Error> {
            // Check file exists - only if NOT dev mode
            if !self.is_dev{
                let file_path = format!("{}/{}.hbs", self.config.templates, template);
                if !Path::new(&file_path).exists() {
                    return fail!(TemplateNotExist, template.to_string());
                }
            }

            // Create route
            self.routes.insert(
                path.to_string(),
                Page::Template {
                    template: template.to_string(),
                    data,
                },
            );

            Ok(self)
        }

        /// Create a route, with raw page content instead of a template
        ///
        /// ## Parameters
        ///
        /// - `path`: The folder (relative to build directory) that file should be written in (`{build}/{path}/index.html`)
        /// - `content`: The raw file contents to write to the file
        <::>
        pub fn route_raw(&mut self, path: &str, content: impl Into<String>) -> &mut Self {
            self.routes.insert(path.to_string(), Page::Raw(content.into()));
            self
        }

        /// Create a route, with raw page content instead of a template
        ///
        /// Adds HTML boilerplate around content (Unlike [`route_raw`](struct.Unreact.html#method.route_raw))
        ///
        /// ## Parameters
        ///
        /// - `path`: The folder (relative to build directory) that file should be written in (`{build}/{path}/index.html`)
        /// - `content`: The raw file contents to write to the file
        <::>
        pub fn route_raw_html(&mut self, path: &str, content: impl Into<String>) -> &mut Self {
            self.routes.insert(path.to_string(), Page::Raw(format!(include_str!("boilerplate.html"), CONTENT = content.into())));
            self
        }

        /// Create the index route
        ///
        /// Alias of `app.route("", ...)`
        ///
        /// File is written to `{build}/index.html`
        ///
        /// **NOTE**: Route will only validate if template exists in production. In dev mode, this function **will always pass**, and error will occur during `run` function
        ///
        /// ## Parameters
        ///
        /// - `template`: The name of the template to use
        /// - `data`: Data to pass into the template, as an `Object`
        <::>
        pub fn index(&mut self, template: &str, data: Object) -> Result<&mut Self, Error> {
            self.route("", template, data)
        }

        /// Create the 404 route
        ///
        /// Alias of `app.route("404", ...)`.
        /// Used as the 404 page, for a path not found
        ///
        /// File is written to `{build}/404/index.html`
        ///
        /// **NOTE**: Route will only validate if template exists in production. In dev mode, this function **will always pass**, and error will occur during `run` function
        ///
        /// ## Parameters
        ///
        /// - `template`: The name of the template to use
        /// - `data`: Data to pass into the template, as an `Object`
        <::>
        pub fn not_found(&mut self, template: &str, data: Object) -> Result<&mut Self, Error> {
            self.route("404", template, data)
        }
    );
}
