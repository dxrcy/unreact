use crate::Port;

/// Html file with javascript websockets to append to every file
///
/// **Should NOT be used without including port** (for parity with `"watch"` feature)
#[cfg(not(feature = "watch"))]
const DEV_SCRIPT_RAW: &str = include_str!("no-watch.html");

/// Html file with javascript (no websockets) to append to every file
///
/// **Should NOT be used without including port**
#[cfg(feature = "watch")]
const DEV_SCRIPT_RAW: &str = include_str!("watch.html");

/// Fallback page, including dev script
///
/// **Should NOT be used without including port**
const FALLBACK_404_RAW: &str = const_str::concat!(include_str!("404.html"), "\n\n", DEV_SCRIPT_RAW);

/// Get dev script, with websocket port included
pub fn dev_script(port_ws: Port) -> String {
    DEV_SCRIPT_RAW.replace("{{PORT}}", &port_ws.to_string())
}

/// Get fallback 404 page, with websocket port included
pub fn fallback_404(port_ws: Port) -> String {
    FALLBACK_404_RAW.replace("{{PORT}}", &port_ws.to_string())
}
