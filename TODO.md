# TODO

-   in config or env ?

    -   server logging ? `UNREACT_LOGS`
    -   ports ? `UNREACT_PORT` `UNREACT_PORT_WS`

## What?

-   Fix ordered list numbers ???

# Maybe

-   Use script file (not inline) for dev script ?

-   Use relative paths for template partials
    -   this will be very hard, if a template using relative paths is used in another template
    -   `/abc` or `abc` - absolute path from `templates/`
    -   `./abc` relative from current template folder
    -   `../abc` super from current
