#[macro_use]
mod macros;
mod error;
mod routes;

pub use serde_json::json;
use serde_json::Value;

pub use crate::error::Error;
use crate::error::MyResult;
use crate::routes::{convert_routes, get_routes, PathToRender, TemplateToRender};

/// Represents json-like object
/// A map of string keys to json values
///
/// A type alias for `serde_json::Map<String, serde_json::Value>`
///
/// See also: [`object`]
pub type Object = serde_json::Map<String, Value>;

//TODO docs
pub fn run(values: Object, _global: Object) -> MyResult<()> {
    let routes = get_routes()?;

    let routes = convert_routes(routes, values)?;

    println!();
    for TemplateToRender { route, paths } in routes {
        println!(
            "\n\x1b[1mUsing template at:\x1b[0m \x1b[33m{}\x1b[0m",
            route.path
        );

        for PathToRender { filepath, values } in paths {
            println!("    • Render to file: \x1b[34m{}\x1b[0m", filepath);
            if !values.is_empty() {
                println!(
                    "        \x1b[2m‣ With values:\x1b[0m \x1b[35m{:?}\x1b[0m",
                    values
                );
            }
        }
    }

    Ok(())
}
