# Unreact

A static site generation framework for Rust using Handlebars and Scss.

Submit issue [here](https://github.com/darccyy/unreact/issues/new)

-   [Docs.rs](https://docs.rs/unreact)
-   [Crates.io](https://crates.io/crates/unreact)

# Usage

For a quick start, check out [Unreact Template](https://github.com/darccyy/unreact-template)

Add the latest version to your `Cargo.toml` dependencies

```toml
unreact = "*"
```

Run with `--dev` or `-d` arguments, for `unreact::is_dev()` function to return `true`

```bash
cargo run -- --dev
```

## Hot-reloading in dev mode

With the `"watch"` feature enabled, the dev server will watch for changes in asset folders (`templates`, `styles`, and `public`; Can be changed with config).
The client will reload if a change was detected.

> NOTE: This will NOT reload the client if Rust files (in `src`) were changed! (See below)

## Watching router in dev mode

This will watch file in `src`, and reload the program. The client should automatically try to reconnect.

```
cargo watch -x "run -- --dev" -w src -w Cargo.toml
```

## Ignoring `"dev"` Feature in Production

Disable default features (`"dev"` and `"watch"`) for less to compile in production.
Code can be the same as in dev build.

```toml
[dependencies]
# Disabled `unreact/dev` by default
unreact = {version = "*", default-features = false}

[features]
# Override `unreact/dev` feature with default features
default = ["unreact/dev"]
```

```bash
# Run with `unreact/dev` feature
cargo run

# Run without `unreact/dev` (for a production server)
cargo run --no-default-features
```

## Small Example

Create an site with a single index page

```rust
use unreact::prelude::*;

fn main() -> Result<(), Error> {
   // Create the app
   // Using default config, not in dev mode, and an example url
   let mut app = Unreact::new(Config::default(), false, "https://example.com")?;
   // Create an index route
   // This uses the template 'page.hbs' in 'templates/'
   // A json object with a value for 'foo' is passed into the template
   app.index("page", object! { foo: "World!" })?;
   // Compile it!
   app.run()
}
```

## Larger Example

```rust
use unreact::prelude::*;

#[test]
fn large_example() -> Result<(), Error> {
    // Custom config
    let config = Config {
        strict: true,
        ..Config::default()
    };

    // Run with `is_dev`
    let mut app = Unreact::new(config, is_dev(), "https://example.com")?;

    // Set the global variable 'smiley'
    app.globalize(object! {
        smiley: "(^_^)"
    });

    // Create some routes
    // Note that these methods will never return an error in dev mode. The error will be handled on `app.run()`
    app.index("page", object! {message: "World!"})?
        .not_found("404", object! {})?
        .route_raw("hello", "this is my hello page".to_string())
        .route("article", "other/article", object! {})?;

    // Compiles normally, or opens a dev server and listens if in dev mode
    app.run()
}
```

## Automatic Compilation with Github Pages

In `.github/workflows/build.yaml`:

```yaml
name: Build

on:
    # Triggers the workflow on push or pull request events but only for the "main" branch
    push:
        branches: ["main"]
    pull_request:
        branches: ["main"]

jobs:
    build:
        runs-on: ubuntu-latest

        steps:
            # Checks-out your repository under $GITHUB_WORKSPACE, so your job can access it
            - name: Checkout 🛎️
              uses: actions/checkout@v3

            # Run compilation script with Rust
            - name: Build 🔧
              run: cargo run --no-default-features

            # Push changes with plugin
            - name: Deploy 🚀
              uses: JamesIves/github-pages-deploy-action@v4
              with:
                  # This must be the build directory
                  folder: ./build
```

---

![Unreact Icon](./icon.png)
