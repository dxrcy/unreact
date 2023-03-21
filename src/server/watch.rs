use std::{
    collections::HashMap,
    path::Path,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::Duration,
};

use chrono::Utc;
use notify::{EventKind, RecursiveMode, Watcher};
use simple_websockets::{Event, Message, Responder};

/// Local port to host websocket hub (on localhost)
pub const WS_PORT: u16 = 3001;

/// Folders in workspace directory to watch for changes
const WATCHED_FOLDERS: &[&str] = &["templates", "styles", "public"];
/// Minimum time to wait, in milliseconds, since the last event, for the websocket hub to send a reload request to the client
const MIN_RECOMPILE_INTERVAL: u32 = 0;
/// Time to wait, in milliseconds, before reading a recently saved file
const FILE_SAVE_WAIT: u64 = 300;

/// Initialize websocket hub, with callback app router, and watch files for changes
pub fn watch<F>(router: F)
where
    F: Fn(),
{
    // Initialize websocket hub
    let event_hub = unwrap!(
        simple_websockets::launch(WS_PORT),
        err: "Failed to initialize websockets on port {} `{err:?}`",
        WS_PORT
    );

    // List of connected clients, with ID and handler
    let clients = Arc::new(Mutex::new(HashMap::<u64, Responder>::new()));

    // Handle client events
    let clients_clone = clients.clone();
    thread::spawn(move || loop {
        // Access clients list mutably
        let mut clients = clients_clone.lock().unwrap();

        // Loop every recent event
        for event in event_hub.drain() {
            match event {
                // Client connected, add to list
                Event::Connect(id, responder) => {
                    println!("Connect #{}", id);
                    clients.insert(id, responder);
                }

                // Client disconnected, remove from list
                Event::Disconnect(id) => {
                    println!("Disconnect #{}", id);
                    clients.remove(&id);
                }

                _ => (),
            }
        }
    });

    // Create event handler (channel)
    let (tx, rx) = channel();

    // Create file watcher
    let mut watcher =
        unwrap!(notify::recommended_watcher(tx), err: "Could not create file watcher `{err:?}`");

    // Watch specific folders
    for folder in WATCHED_FOLDERS {
        unwrap!(
            watcher.watch(Path::new(folder), RecursiveMode::Recursive),
            err: "Could not watch folder '{}' `{err:?}`",
            folder
        )
    }

    let mut last_compile = Utc::now().timestamp();
    loop {
        // If file change event message is ok
        let Ok(Ok(event)) = rx.recv() else {
            continue;
        };

        // If file event is: a created, modified, or removed file
        if !matches!(
            event.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        ) {
            continue;
        }

        // If enough time has passed since last reload
        let now = Utc::now().timestamp();
        if last_compile + (MIN_RECOMPILE_INTERVAL as i64) > now {
            continue;
        }
        last_compile = now;

        // Block thread for some time
        // ? How can this be made better ?
        thread::sleep(Duration::from_millis(FILE_SAVE_WAIT));

        // Run callback router
        router();

        // Loop clients
        let clients = clients.lock().unwrap();
        for (_id, client) in clients.iter() {
            // Send a reload request
            client.send(Message::Text("reload".to_string()));
        }
    }
}
