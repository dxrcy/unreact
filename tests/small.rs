use unreact::prelude::*;

#[test]
fn small_example() {
    let config = Config {
        build: "tests/build".to_string(),
        templates: "tests/templates".to_string(),
        styles: "tests/styles".to_string(),
        public: "tests/public".to_string(),
        ..Config::default()
    };

    let mut app = Unreact::new(config, false, "https://example.com").expect("Could not create app");

    app.index("page", object! {})
        .expect("Could not create route");

    app.run().expect("Could not compile");
}
