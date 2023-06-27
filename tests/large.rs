use unreact::prelude::*;

#[test]
fn large_example() {
    let config = Config {
        strict: true,
        build: "tests/build".to_string(),
        templates: "tests/assets/templates".to_string(),
        styles: "tests/assets/styles".to_string(),
        public: "tests/assets/public".to_string(),
        ..Config::default()
    };

    let mut app = Unreact::new(config, false, "https://example.com").expect("Could not create app");

    app.globalize(object! {
        smiley: "(^_^)"
    });

    app.index("page", object! {message: "World!"})
        .expect("Could not create index route")
        .not_found("404", object! {})
        .expect("Could not create 404 route")
        .route_raw("hello", "this is my hello page".to_string())
        .route("article", "other/article", object! {})
        .expect("Could not create custom route");

    app.run().expect("Could not compile");
}
