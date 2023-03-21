#[macro_use]
mod unwrap;
#[cfg(feature = "watch")]
mod watch;

#[cfg(feature = "watch")]
pub use watch::watch;

use crate::DEV_BUILD_DIR;
use http::{Method, Request, Response, StatusCode};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Server,
};
use std::{convert::Infallible, fs, path::Path};

/// Local port to host dev server (on localhost)
pub const SERVER_PORT: u16 = 3000;

/// Html file with javascript websockets to append to every file
#[cfg(not(feature = "watch"))]
pub const DEV_SCRIPT: &str = include_str!("client-script/dev.html");
/// Html file with javascript (no websockets) to append to every file
#[cfg(feature = "watch")]
pub const DEV_SCRIPT: &str = include_str!("client-script/watch.html");

/// Fallback page, including dev script
const FALLBACK_404: &str = const_str::concat!(include_str!("404.html"), "\n\n", DEV_SCRIPT);

/// Create server and listen on localhost port
///
/// Similar to GitHub Pages router
///
/// Reads file on every request: this should not be a problem for a dev server
pub fn listen() {
    // Create runtime
    let runtime = unwrap!(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build(),
        "Failed to build tokio runtime"
    );

    // Block on server running
    unwrap!(
        runtime.block_on(async {
            // Create service for router
            let make_svc =
                make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(server_router)) });

            // Parse IP address
            let addr = unwrap!(
                const_str::concat!("127.0.0.1:", SERVER_PORT).parse(),
                "Failed to parse constant IP address"
            );

            // Create and start server
            Server::bind(&addr).serve(make_svc).await
        }),
        // Generic runtime error
        err: "Error in server runtime: `{err:?}`"
    );
}

/// Route path to read and return file.
///
/// Accepts '/foo', '/foo.html', and '/foo/index.html' patterns
///
/// If no possible file was found, use 404 route (same as <URL>/404 request).
/// If no custom 404 page was found, use fallback 404 page
async fn server_router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Check if is GET request
    if req.method() == Method::GET {
        // Return corresponding file as body if exists
        if let Some(file) = get_best_possible_file(req.uri().path()) {
            return Ok(Response::new(file));
        }
    }

    // 404 page
    Ok(unwrap!(
        Response::builder().status(StatusCode::NOT_FOUND).body(
            if let Some(file) = get_best_possible_file("404") {
                // If custom 404 page is defined (requesting route `/404`)
                file
            } else {
                // Fallback 404 response
                Body::from(FALLBACK_404)
            },
        ),
        // Should not error
        err: "Failed to build 404 page response `{err:?}`",
    ))
}

/// Loops through files in `possible_path_suffixes` to find best file match, and reads file
///
/// Returns as `Option<Body>`, to allow non-UTF-8 file formats (such as images).
/// Returns `None` if no files were found
///
/// Panics if file exists, but was unable to be read
fn get_best_possible_file(path: &str) -> Option<Body> {
    let possible_suffixes = possible_path_suffixes(path);

    for suffix in possible_suffixes {
        let path = &format!("./{DEV_BUILD_DIR}/{path}/{suffix}");

        // If file exists, and not directory
        if Path::new(path).is_file() {
            // Returns file content as `Body`
            // Automatically parses to string, if is valid UTF-8, otherwise uses buffer
            return Some(Body::from(unwrap!(
                fs::read(path),
                // Should only happen due to insufficient permissions or similar, not 'file not exist' error
                "Could not read file '{}'",
                path
            )));
        }
    }
    None
}

/// Gets the possible path 'suffixes' from the path string
///
/// If path ends with '.html', or starts with '/styles' or '/public', then return a slice of an empty string.
/// This path should refer to a file literally
///
/// Otherwise, return a slice of: an empty string (for a literal file), '.html', and '/index.html' (for file path shorthand).
/// Suffixes are returned in that order, to match a file based on specificity
fn possible_path_suffixes(path: &str) -> &'static [&'static str] {
    if path.ends_with(".html") || path.starts_with("/styles") || path.starts_with("/public") {
        &[""]
    } else {
        &["", ".html", "/index.html"]
    }
}
