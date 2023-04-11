#[macro_use]
mod macros;
mod error;
mod routes;

use serde_json::Value;

pub use crate::error::Error;
use crate::error::MyResult;
use crate::routes::get_routes;

pub fn run(_values: Value, _global: Value) -> MyResult<()> {
    let routes = get_routes()?;

    for route in routes {
        println!("Route: {}", route.path);
    }

    Ok(())
}
