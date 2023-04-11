#[macro_use]
mod macros;
mod error;
mod routes;

pub use serde_json::json;
use serde_json::Value;

pub use crate::error::Error;
use crate::error::MyResult;
use crate::routes::{get_routes, Fragment, Route};

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

    type File = (String, Object);

    let mut all_files: Vec<(Route, Vec<File>)> = Vec::new();

    for route in routes {
        println!("Route: {}", route.path);

        let mut files: Vec<File> = vec![(String::new(), Object::new())];

        for fragment in route.path.clone() {
            match fragment {
                Fragment::Literal(literal) => {
                    println!("    /{}", literal);

                    for (file, _obj) in &mut files {
                        file.push('/');
                        file.push_str(&literal);
                    }
                }
                
                //TODO optimize
                Fragment::Value(name) => {
                    println!("    /{} (Value)", name);

                    let Some(map) = values.get(&name) else {
                        throw!("Value is not given with name '{}'", name);
                    };
                    let Some(map) = map.as_object() else {
                        throw!("Value given is not an object map, named '{}'", name);
                    };

                    let mut new_files = Vec::new();

                    for (file, obj) in files {
                        for (key, value) in map {
                            let mut file_new = file.clone();

                            file_new.push('/');
                            file_new.push_str(key);

                            let mut obj_new = obj.clone();
                            obj_new.insert(name.clone(), value.to_owned());

                            new_files.push((file_new, obj_new));
                        }
                    }

                    files = new_files;
                }
            }
        }

        all_files.push((route, files));
    }

    println!();
    for file in all_files {
        println!("{:#?}\n", file);
    }

    Ok(())
}
