# TODO

-   Add config to clear terminal on every recompile
-   Client auto reconnect: try frequently, then after (10?) attempts, try less frequently
-   Add tests for each feature (compile check only?)

-   Add doc comments to (private) struct fields
-   Add reference to handlebars docs in handlebars mentions in doc comments

## What?

-   Fix ordered list numbers ???

# Maybe

-   Use cargo.toml package name in print message ?

-   Use relative paths for template partials
    -   this will be very hard, if a template using relative paths is used in another template
    -   `/abc` or `abc` - absolute path from `templates/`
    -   `./abc` relative from current template folder
    -   `../abc` super from current
