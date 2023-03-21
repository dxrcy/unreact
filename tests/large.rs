use unreact::prelude::*;

#[test]
fn large_example() {
    let config = Config {
        strict: true,
        ..Config::default()
    };

    let mut app = Unreact::new(config, false, "https://example.com").expect("Could not create app");

    app.globalize(object! {
        smiley: "(^_^)"
    });

    app.index("page", object! {message: "World!"})
        .not_found("404", object! {})
        .route_raw("hello", "this is my hello page".to_string())
        .route_bare("article", "other/article");

    app.compile().expect("Could not compile");
}
