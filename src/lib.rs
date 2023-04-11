#[macro_use]
mod macros;
mod error;
mod files;
mod routes;

use std::fs;

use handlebars::Handlebars;
pub use serde_json::json;
use serde_json::Value;

pub use crate::error::Error;
use crate::error::MyResult;
use crate::files::{create_build_dir, create_dir_all_for_file};
use crate::routes::{convert_routes, get_routes, PathToRender, TemplateToRender};

/// Represents json-like object
/// A map of string keys to json values
///
/// A type alias for `serde_json::Map<String, serde_json::Value>`
///
/// See also: [`object`]
pub type Object = serde_json::Map<String, Value>;

//TODO docs
pub fn run(global: Object) -> MyResult {
    let is_dev = true;
    let strict_mode = true;

    let dir_dist = "dist";
    let dir_build = format!("{}/{}", dir_dist, if is_dev { "dev" } else { "build" });
    let dir_assets = "assets";
    let dir_routes = format!("{}/routes", dir_assets);

    create_build_dir(&dir_build)?;

    //TODO check assets dirs

    let routes = get_routes(&dir_routes)?;
    let routes = convert_routes(routes, &global)?;

    let mut registry = Handlebars::new();
    registry.set_strict_mode(strict_mode);

    for TemplateToRender { route, .. } in &routes {
        let template_name = route.path.to_string();
        try_else!(
            try registry.register_partial(&template_name, route.template.clone()),
            else err: throw!("Failed to register template named '{}': {err:?}", template_name),
        );
    }

    println!();
    for TemplateToRender { route, paths } in routes {
        println!(
            "\n\x1b[1mUsing template at:\x1b[0m \x1b[33m{}\x1b[0m",
            route.path
        );

        for PathToRender {
            filepath,
            mut values,
        } in paths
        {
            println!("    • Render to file: \x1b[34m{}\x1b[0m", filepath);
            if !values.is_empty() {
                println!(
                    "        \x1b[2m‣ With values:\x1b[0m \x1b[35m{:?}\x1b[0m",
                    values
                );
            }

            // Add global to value
            let mut object = global.clone();
            object.insert("self".to_string(), json!(values));
            values = object;

            let template_name = route.path.to_string();
            let rendered = try_else!(
                try registry.render(&template_name, &values),
                else err: throw!("Failed to render template named '{}': {err:?}", template_name),
            );

            let path = format!("{}{}", dir_build, filepath);
            create_dir_all_for_file(&path)?;
            try_else!(
                try fs::write(&path, rendered),
                else err: throw!("[io] Failed to write file to '{}': {err:?}", path)
            );
        }
    }

    Ok(())
}
