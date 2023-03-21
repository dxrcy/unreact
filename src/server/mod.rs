#[macro_use]
mod unwrap;
#[cfg(feature = "watch")]
mod watch;

#[cfg(feature = "watch")]
pub use watch::watch;

use std::{convert::Infallible, fs, path::Path};

use http::{Method, Request, Response, StatusCode};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Server,
};

use crate::DEV_BUILD_DIR;

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

/// Create server and listen on local port
///
/// Almost mimics GitHub Pages
///
/// Reads file on every GET request, however this should not be a problem for a dev server
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
        err: "Error in server runtime: `{err:?}`"
    );
}

/// Route path to read and return file
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
        err: "Failed to build 404 page response `{err:?}`",
    ))
}

/// Loops through files in `possible_files_from_path` to find best file match
///
/// Returns `None` if no file was founds
///
/// Returns as `Option<Body>`, to allow non-UTF-8 file formats (such as images)
///
/// Panics if file exists, but was unable to be read
fn get_best_possible_file(path: &str) -> Option<Body> {
    // Convert request to possible filepaths
    let possible_path_suffix = possible_files_from_path(path);

    for suffix in possible_path_suffix {
        let path = &format!("./{DEV_BUILD_DIR}/{path}/{suffix}");

        // If file exists, and not directory
        if Path::new(path).is_file() {
            // Returns file content as `Body`
            // Automatically parses to string, if is valid UTF-8, otherwise uses buffer
            return Some(Body::from(unwrap!(
                fs::read(path),
                "Could not read file '{}'",
                path
            )));
        }
    }
    None
}

/// Converts path from request into possible files to correspond to
///
/// If path ends with `.html`, or starts with `/styles` or `/public`, returns path, unchanged
///
/// Else returns path + `.html`, and path + `/index.html`
///
/// All file paths returned are relative to workspace directory, and include dev build path
fn possible_files_from_path(path: &str) -> &'static [&'static str] {
    if path.ends_with(".html") || path.starts_with("/styles") || path.starts_with("/public") {
        &[""]
    } else {
        &["", ".html", "/index.html"]
    }
}
