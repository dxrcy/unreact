use unreact::prelude::*;

#[test]
fn small_example() {
    let config = Config {
        build: "tests/build".to_string(),
        templates: "tests/assets/templates".to_string(),
        styles: "tests/assets/styles".to_string(),
        public: "tests/assets/public".to_string(),
        ..Config::default()
    };

    let mut app = Unreact::new(config, false, "https://example.com").expect("Could not create app");

    app.index("page", object! {})
        .expect("Could not create route");

    app.run().expect("Could not compile");
}
