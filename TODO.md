# TODO

## What?

-   Fix ordered list numbers ???

# Maybe

-   Remove `app.route_bare` ?
-   Use script file (not inline) for dev script ?

-   Use env var for server logging ? `UNREACT_LOGS`
-   Use env var for ports ? `UNREACT_PORT` `UNREACT_PORT_WS`

-   Use relative paths for template partials
    -   this will be very hard, if a template using relative paths is used in another template
    -   `/abc` or `abc` - absolute path from `templates/`
    -   `./abc` relative from current template folder
    -   `../abc` super from current
