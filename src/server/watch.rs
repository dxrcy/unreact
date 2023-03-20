use std::{
    collections::HashMap,
    path::Path,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::Duration,
};

use chrono::Utc;
use notify::{EventKind, RecursiveMode, Watcher};
use simple_websockets::{Event, Responder};

pub const WS_PORT: u16 = 3001;

const MIN_RECOMPILE_INTERVAL: u32 = 0;

/// Initialize websocket server, with callback router, and watch files for changes
//TODO Separate stuff
pub fn watch<F>(router: F)
where
    F: Fn(),
{
    let event_hub = simple_websockets::launch(WS_PORT)
        .unwrap_or_else(|_| panic!("Failed to initialize websockets on port {}", WS_PORT));

    let clients = Arc::new(Mutex::new(HashMap::<u64, Responder>::new()));

    let clients_clone = clients.clone();
    thread::spawn(move || loop {
        let mut clients = clients_clone.lock().unwrap();

        for event in event_hub.drain() {
            match event {
                Event::Connect(id, responder) => {
                    println!("Connect #{}", id);
                    clients.insert(id, responder);
                }

                Event::Disconnect(id) => {
                    println!("Disconnect #{}", id);
                    clients.remove(&id);
                }

                _ => (),
            }
        }
    });

    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(tx).expect("Could not create watcher");

    let folders = ["templates", "styles", "public"];
    for folder in folders {
        watcher
            .watch(Path::new(folder), RecursiveMode::Recursive)
            .unwrap_or_else(|_| panic!("Could not watch folder '{}'", folder));
    }

    let mut last_compile = Utc::now().timestamp();

    let clients_clone = clients;
    loop {
        let event = rx.recv().expect("idk! #1").expect("idk! #2");

        if matches!(
            event.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        ) {
            let now = Utc::now().timestamp();

            if last_compile + (MIN_RECOMPILE_INTERVAL as i64) < now {
                last_compile = now;

                // ???? why ????
                thread::sleep(Duration::from_millis(300));

                router();

                let clients_ref = clients_clone.lock().unwrap();

                for (_id, client) in clients_ref.iter() {
                    client.send(simple_websockets::Message::Text("reload".to_string()));
                }
            }
        }
    }
}