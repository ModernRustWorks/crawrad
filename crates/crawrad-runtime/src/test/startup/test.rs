//! The generated app tells the person why it did not start.

use crate::startup::{startup_failure_text, write_startup_log};

fn scratch(name: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let dir = std::env::temp_dir().join(format!(
        "crawrad-startup-{name}-{}-{nanos:x}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("scratch folder");
    dir
}

#[test]
fn the_message_carries_the_error_and_where_the_log_is() {
    let text = startup_failure_text(
        "Sales",
        &"no suitable graphics adapter",
        Some(std::path::Path::new("C:/apps/sales.error.log")),
    );
    assert!(text.starts_with("Sales could not start."), "{text}");
    assert!(text.contains("no suitable graphics adapter"), "{text}");
    assert!(text.contains("C:/apps/sales.error.log"), "{text}");
}

#[test]
fn without_a_log_the_message_does_not_point_anywhere() {
    let text = startup_failure_text("Sales", &"boom", None);
    assert!(!text.contains("saved to"), "{text}");
}

#[test]
fn the_log_lands_beside_the_exe() {
    let dir = scratch("log");
    let path = dir.join("sales.error.log");
    let written = write_startup_log(&path, "Sales", &"boom");
    assert_eq!(written.as_deref(), Some(path.as_path()));
    let body = std::fs::read_to_string(&path).expect("log written");
    assert!(
        body.contains("Sales could not start.") && body.contains("boom"),
        "{body}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn an_unwritable_folder_falls_back_to_temp() {
    // A path whose folder does not exist stands for the read-only
    // `Program Files` case: the first write fails, the fallback must not.
    let dir = scratch("fallback");
    let name = format!("crawrad-startup-fallback-{}.error.log", std::process::id());
    let path = dir.join("missing").join(&name);
    let written = write_startup_log(&path, "Sales", &"boom").expect("fallback written");
    assert_eq!(written, std::env::temp_dir().join(&name));
    assert!(written.exists());
    let _ = std::fs::remove_file(written);
    let _ = std::fs::remove_dir_all(dir);
}
