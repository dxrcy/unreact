#[macro_use]
mod macros;
mod error;
mod routes;

pub use serde_json::json;
use serde_json::Value;

pub use crate::error::Error;
use crate::error::MyResult;
use crate::routes::{convert_routes, get_routes};

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
    for file in routes {
        println!("{:?}\n", file);
    }

    Ok(())
}
