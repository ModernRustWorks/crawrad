//! Tests for the project file watcher and ignore filters.

use std::fs;
use std::path::Path;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::watch::{ProjectWatcher, ignored};

#[test]
fn ignored_pure_decisions() {
    let root = Path::new("root");
    assert!(ignored(root, Path::new("root/target/debug/x")));
    assert!(ignored(root, Path::new("root/.git/HEAD")));
    assert!(ignored(root, Path::new("root/.crawrad/history/entry")));
    assert!(ignored(root, Path::new("root/a/target/y")));
    assert!(ignored(root, Path::new("root/a/.crawrad/history")));
    assert!(!ignored(root, Path::new("root/src/main.rs")));
    assert!(!ignored(root, Path::new("root/targets.rs")));
}

#[test]
fn watcher_detects_changes_and_filters_target() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let unique = format!("crawrad_watch_test_{}_{now}", std::process::id());
    let temp_root = std::env::temp_dir().join(unique);
    fs::create_dir_all(&temp_root).unwrap();

    let target_dir = temp_root.join("target");
    fs::create_dir_all(&target_dir).unwrap();

    let mut watcher = ProjectWatcher::start(&temp_root).expect("failed to start watcher");
    assert_eq!(watcher.root(), temp_root.as_path());

    // Brief sleep to let directory registration settle
    std::thread::sleep(Duration::from_millis(100));

    let normal_file = temp_root.join("hello.rs");
    let target_file = target_dir.join("ignored.txt");

    fs::write(&target_file, "ignored text").unwrap();
    fs::write(&normal_file, "fn main() {}").unwrap();

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut found_normal = false;
    let mut found_target = false;

    while Instant::now() < deadline {
        let changed = watcher.drain();
        for p in changed {
            if p.ends_with("hello.rs") {
                found_normal = true;
            }
            if p.ends_with("ignored.txt") {
                found_target = true;
            }
        }
        if found_normal {
            std::thread::sleep(Duration::from_millis(200));
            for p in watcher.drain() {
                if p.ends_with("ignored.txt") {
                    found_target = true;
                }
            }
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    let _ = fs::remove_dir_all(&temp_root);

    assert!(found_normal, "watcher should report modified file hello.rs");
    assert!(
        !found_target,
        "watcher should never report target/ignored.txt"
    );
}
