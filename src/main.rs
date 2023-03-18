use unreact::{object, App, Config};

fn main() {
    let config = Config {
        strict: true,
        ..Config::default()
    };

    let mut app = App::new(config, true, "https://bruh.news/").expect("Could not initialize app");

    // println!("{:#?}", app);

    app.set_globals(object! {
        debug: ":)"
    });

    app.index(
        "page",
        object! {
            world: "World"
        },
    )
    .expect("Could not create page");

    // println!("{:#?}", app);

    app.finish().expect("Could not finish app");
}
