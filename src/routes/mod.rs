mod files;
mod path;

use self::files::load_filemap;
use crate::error::MyResult;

pub use self::path::{Fragment, RoutePath};

#[derive(Debug)]
pub struct Route {
    pub path: RoutePath,
    pub contents: String,
}

/// Get routes from directory
//TODO Check for duplicates
pub fn get_routes() -> MyResult<Vec<Route>> {
    let filemap = load_filemap("assets/routes")?;

    // Must be for loop, as error may be returned
    let mut routes = Vec::new();
    for (filepath, contents) in filemap {
        routes.push(Route {
            path: RoutePath::try_from(filepath.as_str())?,
            contents,
        })
    }

    Ok(routes)
}
