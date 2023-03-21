use unreact::prelude::*;

fn main() -> Result<(), Error> {
    let mut app = Unreact::new(Config::default(), false, "https://example.com")?;

    app.index("page", object! {message: "World!"})
        .not_found("404", object! {})
        .route_raw("hello", "this is my hello page".to_string())
        .route_bare("article", "other/article");

    app.compile()
}
