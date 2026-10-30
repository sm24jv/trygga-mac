use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Appends a line with the time of this moment to a log in `TRYGGA_MEASURE_DIR`. Without that
/// variable it does nothing, so only a measuring run writes anything.
pub fn append(log: &str, line: &str) {
    let Ok(dir) = env::var("TRYGGA_MEASURE_DIR") else {
        return;
    };
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_millis());
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(Path::new(&dir).join(log));
    if let Ok(mut file) = file {
        let _ = writeln!(file, "{now_ms} {line}");
    }
}
