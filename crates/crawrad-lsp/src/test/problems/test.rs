//! The problems per file, from what `publishDiagnostics` left: errors and
//! warnings counted, a file with neither left out, the last answer wins.

use super::{file_key, problem_counts_of};
use std::collections::HashMap;
use std::sync::Mutex;

fn publish(kept: &Mutex<HashMap<String, Vec<serde_json::Value>>>, file: &str, severities: &[u64]) {
    let list: Vec<_> = severities
        .iter()
        .map(|s| serde_json::json!({ "severity": s, "message": "m" }))
        .collect();
    let key = file_key(std::path::Path::new(&format!("C:/p/src/{file}")));
    kept.lock().unwrap().insert(key, list);
}

#[test]
fn errors_and_warnings_are_counted_per_file() {
    let kept = Mutex::default();
    publish(&kept, "a.rs", &[1, 2, 2, 3]);
    publish(&kept, "b.rs", &[2]);
    publish(&kept, "c.rs", &[3, 4]);
    let counts = problem_counts_of(&kept.lock().unwrap());
    let short: Vec<_> = counts
        .iter()
        .map(|(f, e, w)| (f.rsplit('/').next().unwrap_or(""), *e, *w))
        .collect();
    assert_eq!(
        short,
        [("a.rs", 1, 2), ("b.rs", 0, 1)],
        "hints and information are not problems"
    );
    // fixed: the server publishes an empty list, and the file is clean
    publish(&kept, "a.rs", &[]);
    let counts = problem_counts_of(&kept.lock().unwrap());
    assert_eq!(counts.len(), 1);
}
