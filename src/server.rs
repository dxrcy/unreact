use std::{
    collections::HashMap,
    convert::Infallible,
    fs,
    path::Path,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

// use chrono::Utc;
use http::{Method, Request, Response, StatusCode};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Server,
};
use notify::{EventKind, RecursiveMode, Watcher};
use simple_websockets::{Event, Responder};

use crate::DEV_BUILD_DIR;

/// Local address with port to host dev server
pub const SERVER_PORT: u16 = 3000;
pub const SERVER_ADDRESS: &str = const_str::concat!("127.0.0.1:", SERVER_PORT);
pub const WS_PORT: u16 = 3001;

// pub const RECOMPILE_DELAY_SECS: i64 = 0;

/// Partial for 'hot reloading' document in development
pub const DEV_SCRIPT: &str = include_str!("dev.html");

/// Open file server, watch source files to hot reload client
pub fn watch<F>(router: F)
where
    F: Fn(),
{
    // Create file server on new thread
    thread::spawn(init_server);

    // Create websocket server
    init_websocket(router);
}

/// Initialize websocket server, with callback router
fn init_websocket<F>(router: F)
where
    F: Fn(),
{
    let event_hub = simple_websockets::launch(WS_PORT).expect(&format!(
        "Failed to initialize websockets on port {}",
        WS_PORT
    ));

    let clients = Arc::new(Mutex::new(HashMap::<u64, Responder>::new()));

    let clients_clone = clients.clone();
    thread::spawn(move || loop {
        let mut clients = clients_clone.lock().unwrap();

        for event in event_hub.drain() {
            match event {
                Event::Connect(id, responder) => {
                    clients.insert(id, responder);
                }

                Event::Disconnect(id) => {
                    clients.remove(&id);
                }

                Event::Message(_id, _msg) => (),
            }
        }
    });

    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(tx).expect("Could not create watcher");

    let folders = ["templates", "styles", "public"];
    for folder in folders {
        watcher
            .watch(Path::new(folder), RecursiveMode::Recursive)
            .expect(&format!("Could not watch folder '{}'", folder));
    }

    // let mut last_compile = Utc::now().timestamp();

    let clients_clone = clients.clone();
    loop {
        let event = rx.recv().expect("idk! #1").expect("idk! #2");

        if matches!(
            event.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        ) {
            // let now = Utc::now().timestamp();

            // if last_compile + RECOMPILE_DELAY_SECS < now {
                // last_compile = now;

                router();

                let clients_ref = clients_clone.lock().unwrap();

                for (_id, client) in clients_ref.iter() {
                    client.send(simple_websockets::Message::Text("reload".to_string()));
                }
            // }
        }
    }
}

/// Create server and listen on local port
///
/// Almost mimics GitHub Pages
///
/// Reads file on every GET request, however this should not be a problem for a dev server
pub fn init_server() {
    // Start `tokio` runtime (without macro)
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(async {
            // Create service for router
            let make_svc =
                make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(server_router)) });

            // Create server
            let addr = SERVER_ADDRESS.parse().expect("Invalid IP address");
            let server = Server::bind(&addr).serve(make_svc);

            // Start server
            println!("Listening on http://{}", addr);
            server.await?;

            Ok::<_, hyper::Error>(())
        })
        .expect("Error in Runtime");
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
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(
            // If custom 404 page is defined
            if let Some(file) = get_best_possible_file("404") {
                // Custom 404 page using request `/404`
                return Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(file)
                    .unwrap());
            } else {
                //TODO This will not have 404 status????
                //TODO Also no dev script!

                // Fallback 404 response
                "404 - File not found. Custom 404 page not found.".to_string()
            },
        ))
        .unwrap())
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
    let possible_files = possible_files_from_path(path);
    for file in &possible_files {
        let file = &format!("./{DEV_BUILD_DIR}/{file}");
        // If file exists, and not directory
        if Path::new(file).is_file() {
            // Returns file content as `Body`
            // Automatically parses to string, if is valid UTF-8, otherwise uses buffer
            return Some(Body::from(
                fs::read(file).unwrap_or_else(|_| panic!("Could not read file '{file}'")),
            ));
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
fn possible_files_from_path(path: &str) -> Vec<String> {
    if path.ends_with(".html") || path.starts_with("/styles") || path.starts_with("/public") {
        vec![path.to_string()]
    } else {
        vec![
            path.to_string(),
            path.to_string() + ".html",
            path.to_string() + "/index.html",
        ]
    }
}
