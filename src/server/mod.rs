#[macro_use]
mod unwrap;
mod files;
#[cfg(feature = "watch")]
mod watch;

#[cfg(feature = "watch")]
pub use watch::watch;

pub use files::{dev_script, fallback_404};

use std::{convert::Infallible, fs, path::Path};

use http::{Method, Request, Response, StatusCode};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Server,
};

use crate::{Port, DEV_BUILD_DIR};

/// Create server and listen on localhost port
///
/// Similar to GitHub Pages router
///
/// Reads file on every request: this should not be a problem for a dev server
pub fn listen(port: Port, port_ws: Port) {
    // Create runtime
    let runtime = unwrap!(
        tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build(),
        "Failed to build tokio runtime"
    );

    // Block on server running
    unwrap!(
        runtime.block_on(async {
            // Create service for router
            let port_ws = port_ws.clone();
            let make_svc =
                make_service_fn(|_| async move {
                    Ok::<_, Infallible>(service_fn(move |req| server_router(req, port_ws)))
                });

            // Parse IP address
            let addr = unwrap!(
                format!("127.0.0.1:{}", port).parse(),
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
async fn server_router(req: Request<Body>, port_ws: Port) -> Result<Response<Body>, Infallible> {
    // Check if is GET request
    if req.method() == Method::GET {
        // Return corresponding file as body if exists
        if let Some(file) = get_best_possible_file(req.uri().path()) {
            return Ok(Response::new(file));
        }
    }

    // 404 route
    Ok(unwrap!(
        Response::builder().status(StatusCode::NOT_FOUND).body(
            if let Some(file) = get_best_possible_file("/404.html") {
                // If custom 404 route is defined (requesting route `/404.html`)
                file
            } else {
                // Fallback 404 response
                Body::from(fallback_404(port_ws))
            },
        ),
        // Should not error
        err: "Failed to build 404 route response `{err:?}`",
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
        let path = &format!("{DEV_BUILD_DIR}{path}{suffix}");

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
    if path.ends_with(".html") || path.starts_with("/styles/") || path.starts_with("/public/") {
        &[""]
    } else {
        &["", ".html", "/index.html"]
    }
}
