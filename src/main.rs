use unreact::prelude::*;

fn main() -> Result<(), Error> {
    let is_dev = true;

    let config = Config {
        strict: true,
        ..Config::default()
    };

    let mut app = Unreact::new(config, is_dev, "https://bruh.news/").expect("Could not create app");

    app.globalize(object! {
        debug: "(^_^)"
    });

    app.index("page", object! {message: "World"})
        .route_raw("hello", "this is my hello page".to_string())
        .route_bare("article", "other/article");

    app.run().expect("Could not compile app");

    println!("Compiled successfully");

    Ok(())
}
