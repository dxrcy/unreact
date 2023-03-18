use unreact::{object, Config, Unreact};

fn main() -> Result<(), String> {
    let is_dev = true;

    let config = Config {
        strict: true,
        ..Config::default()
    };

    let mut app = Unreact::new(config, is_dev, "https://bruh.news/")?;

    app.globalize(object! {
        debug: "(^_^)"
    });

    app.index("page", object! {message: "World"})
        .route_exact("hello", "this is my hello page".to_string())
        .route_bare("article", "other/article");

    app.run()?;

    println!("Compiled successfully");

    Ok(())
}
