use unreact::prelude::*;

fn main() -> Result<(), Error> {
    let config = Config {
        strict: true,
        ..Config::default()
    };

    let mut app = Unreact::new(
        config,
        is_dev(),
        "file:///C:/Users/darcy/Documents/code/unreact/build",
    )?;

    app.index("page", object! {message: "World!"})
        .not_found("404", object! {})
        .route_raw("hello", "this is my hello page".to_string())
        .route_bare("article", "other/article");

    app.run()?;

    println!("Compiled");

    Ok(())
}
