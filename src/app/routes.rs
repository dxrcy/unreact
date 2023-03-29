use crate::{Object, Page, Unreact};

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
            /// # Routing Methods
            ///
            /// - [`route`](struct.Unreact.html#method.route): Create a normal route
            /// - [`route_raw`](struct.Unreact.html#method.route_raw): Create a route without a template
            /// - [`route_bare`](struct.Unreact.html#method.route_bare): Create a route without any data
            /// - [`index`](struct.Unreact.html#method.index): Create an index route (`/`)
            /// - [`not_found`](struct.Unreact.html#method.not_found): Create a 404 route (`/404`)
            $item
        )*
    };
}

impl Unreact {
    include_shared_docs!(
        /// Create a route
        ///
        /// ## Parameters
        ///
        /// - `path`: The folder (relative to build directory) that file should be written in (`{build}/{path}/index.html`)
        /// - `template`: The name of the template to use
        /// - `data`: Data to pass into the template, as an `Object`
        <::>
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

        /// Create a route without any data given to the template
        ///
        /// ## Parameters
        ///
        /// - `path`: The folder (relative to build directory) that file should be written in (`{build}/{path}/index.html`)
        /// - `template`: The name of the template to use
        <::>
        pub fn route_bare(&mut self, path: &str, template: &str) -> &mut Self {
            self.route(path, template, object! {})
        }

        /// Create a route, with raw page content instead of a template
        ///
        /// ## Parameters
        ///
        /// - `path`: The folder (relative to build directory) that file should be written in (`{build}/{path}/index.html`)
        /// - `content`: The raw file contents to write to the file
        <::>
        pub fn route_raw(&mut self, path: &str, content: impl Into<String>) -> &mut Self {
            self.pages.insert(path.to_string(), Page::Raw(content.into()));
            self
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
        <::>
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
        <::>
        pub fn not_found(&mut self, template: &str, data: Object) -> &mut Self {
            self.route("404", template, data)
        }
    );
}
