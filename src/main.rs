use unreact::{object, Config, Unreact};

fn main() -> Result<(), String> {
    let is_dev = true;

    let router = || {
        // println!("Compiling...");

        let config = Config {
            strict: true,
            ..Config::default()
        };

        let mut app =
            Unreact::new(config, is_dev, "https://bruh.news/").expect("Could not initialize app");

        // println!("{:#?}", app);

        app.set_globals(object! {
            debug: ":)"
        });

        let rendered = app.render_empty("page").expect("Could not render");

        // println!("{:?}", rendered);

        app.page_plain("", rendered);

        // app.index(
        //     "page",
        //     object! {
        //         world: "World"
        //     },
        // )
        // .expect("Could not create page");

        // println!("{:#?}", app);

        app.finish().expect("Could not finish app");
    };

    unreact::run(router, is_dev);

    println!("Compiled successfully");

    Ok(())
}
