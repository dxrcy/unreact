use unreact::prelude::*;

#[test]
fn small_example() {
    let mut app = Unreact::new(Config::default(), false, "https://example.com")
        .expect("Could not create app");

    app.index("page", object! {});

    app.compile().expect("Could not compile");
}
