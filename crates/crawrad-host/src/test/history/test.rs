//! Unit tests for local file history engine.

use std::path::PathBuf;

use super::{HistorySettings, LocalHistory, Origin};

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(test_name: &str) -> Self {
        let unique = format!(
            "crawrad_history_test_{}_{}_{}",
            std::process::id(),
            test_name,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        );
        let path = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).ok();
    }
}

#[test]
fn snapshot_records_three_entries_in_order() {
    let temp = TempDir::new("snapshot");
    let history = LocalHistory::new(temp.path.clone(), HistorySettings::default());
    let file = temp.path.join("src").join("main.rs");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();

    let text_a = "first content";
    let text_b = "second content";
    let text_c = "third content";

    let recorded_a = history.record(&file, text_a, Origin::Save, 100).unwrap();
    let recorded_b = history
        .record(&file, text_b, Origin::External, 200)
        .unwrap();
    let recorded_c = history.record(&file, text_c, Origin::Save, 300).unwrap();

    assert!(recorded_a);
    assert!(recorded_b);
    assert!(recorded_c);

    let entries = history.entries(&file);
    assert_eq!(entries.len(), 3);

    assert_eq!(entries[0].time, 100);
    assert_eq!(entries[0].origin, Origin::Save);
    assert_eq!(
        history.content(&file, &entries[0]),
        Some(text_a.to_string())
    );

    assert_eq!(entries[1].time, 200);
    assert_eq!(entries[1].origin, Origin::External);
    assert_eq!(
        history.content(&file, &entries[1]),
        Some(text_b.to_string())
    );

    assert_eq!(entries[2].time, 300);
    assert_eq!(entries[2].origin, Origin::Save);
    assert_eq!(
        history.content(&file, &entries[2]),
        Some(text_c.to_string())
    );
}

#[test]
fn deduplication_avoids_duplicate_consecutive_entries_and_blobs() {
    let temp = TempDir::new("dedup");
    let history = LocalHistory::new(temp.path.clone(), HistorySettings::default());
    let file = temp.path.join("src").join("lib.rs");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();

    let text_a = "code A";
    let text_b = "code B";

    let recorded_1 = history.record(&file, text_a, Origin::Save, 100).unwrap();
    let recorded_2 = history.record(&file, text_a, Origin::Save, 101).unwrap();
    assert!(recorded_1);
    assert!(!recorded_2);

    let entries = history.entries(&file);
    assert_eq!(entries.len(), 1);

    let recorded_3 = history
        .record(&file, text_b, Origin::External, 200)
        .unwrap();
    let recorded_4 = history.record(&file, text_a, Origin::Save, 300).unwrap();
    assert!(recorded_3);
    assert!(recorded_4);

    let entries = history.entries(&file);
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].hash, entries[2].hash);

    let history_dir = temp.path.join(".crawrad").join("history");
    let mut blob_count = 0;
    for entry in std::fs::read_dir(&history_dir).unwrap().flatten() {
        if entry.path().is_dir() {
            for sub_entry in std::fs::read_dir(entry.path()).unwrap().flatten() {
                if sub_entry.path().extension().and_then(|s| s.to_str()) == Some("txt") {
                    blob_count += 1;
                }
            }
        }
    }
    assert_eq!(blob_count, 2);
}

#[test]
fn limit_pruning_removes_oldest_entries_and_unreferenced_blobs() {
    let temp = TempDir::new("prune");
    let history = LocalHistory::new(
        temp.path.clone(),
        HistorySettings {
            limit: 2,
            ..Default::default()
        },
    );
    let file = temp.path.join("doc.txt");

    let text_a = "alpha";
    let text_b = "beta";
    let text_c = "gamma";

    history.record(&file, text_a, Origin::Save, 10).unwrap();
    history.record(&file, text_b, Origin::Save, 20).unwrap();

    let entries_before = history.entries(&file);
    assert_eq!(entries_before.len(), 2);
    let hash_a = entries_before[0].hash.clone();
    let hash_b = entries_before[1].hash.clone();

    // Recording C must prune A since limit is 2
    history.record(&file, text_c, Origin::Save, 30).unwrap();

    let entries = history.entries(&file);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].time, 20);
    assert_eq!(entries[0].hash, hash_b);
    assert_eq!(entries[1].time, 30);
    assert_eq!(
        history.content(&file, &entries[0]),
        Some(text_b.to_string())
    );
    assert_eq!(
        history.content(&file, &entries[1]),
        Some(text_c.to_string())
    );

    let dir_hash = super::sha256_hex(b"doc.txt");
    let blob_a = temp
        .path
        .join(".crawrad")
        .join("history")
        .join(dir_hash)
        .join(format!("{hash_a}.txt"));
    assert!(!blob_a.exists());
}

#[test]
fn file_outside_root_records_nothing() {
    let temp = TempDir::new("outside_root");
    let outside = TempDir::new("outside_other");
    let history = LocalHistory::new(temp.path.clone(), HistorySettings::default());
    let outside_file = outside.path.join("outside.rs");

    let recorded = history
        .record(&outside_file, "secret", Origin::Save, 100)
        .unwrap();
    assert!(!recorded);
    assert!(history.entries(&outside_file).is_empty());
    assert!(!temp.path.join(".crawrad").exists());
}

#[test]
fn gitignore_appended_once_when_existing() {
    let temp = TempDir::new("gitignore_existing");
    let gitignore = temp.path.join(".gitignore");
    std::fs::write(&gitignore, "target\n").unwrap();

    let history = LocalHistory::new(temp.path.clone(), HistorySettings::default());
    let file = temp.path.join("file.rs");

    history.record(&file, "first", Origin::Save, 1).unwrap();
    history.record(&file, "second", Origin::Save, 2).unwrap();

    let content = std::fs::read_to_string(&gitignore).unwrap();
    let lines: Vec<&str> = content
        .lines()
        .map(|s| s.trim())
        .filter(|s| *s == ".crawrad/")
        .collect();
    assert_eq!(lines.len(), 1);
}

#[test]
fn gitignore_not_created_when_no_git_nor_gitignore() {
    let temp = TempDir::new("no_git");
    let history = LocalHistory::new(temp.path.clone(), HistorySettings::default());
    let file = temp.path.join("file.rs");

    history.record(&file, "hello", Origin::Save, 1).unwrap();

    let gitignore = temp.path.join(".gitignore");
    assert!(!gitignore.exists());
}

#[test]
fn gitignore_created_when_git_dir_exists() {
    let temp = TempDir::new("git_exists");
    std::fs::create_dir_all(temp.path.join(".git")).unwrap();
    let history = LocalHistory::new(temp.path.clone(), HistorySettings::default());
    let file = temp.path.join("file.rs");

    history.record(&file, "hello", Origin::Save, 1).unwrap();

    let gitignore = temp.path.join(".gitignore");
    assert!(gitignore.exists());
    let content = std::fs::read_to_string(&gitignore).unwrap();
    assert_eq!(content.trim(), ".crawrad/");
}

#[test]
fn disk_watch_saved_records_entry_under_history() {
    let temp = TempDir::new("disk_watch_saved");
    let file = temp.path.join("saved_file.rs");
    std::fs::write(&file, "x").unwrap();

    let mut disk_watch = crate::disk_watch::DiskWatch::default();
    disk_watch.ensure(Some(temp.path.clone()));

    disk_watch.saved(&file, "x".to_string());

    let history = LocalHistory::new(temp.path.clone(), HistorySettings::default());
    let entries = history.entries(&file);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].origin, Origin::Save);
    assert_eq!(history.content(&file, &entries[0]), Some("x".to_string()));
}

#[test]
fn disabled_history_records_nothing() {
    let temp = TempDir::new("disabled");
    let history = LocalHistory::new(
        temp.path.clone(),
        HistorySettings {
            enabled: false,
            ..Default::default()
        },
    );
    let file = temp.path.join("file.rs");
    let recorded = history.record(&file, "code", Origin::Save, 100).unwrap();
    assert!(!recorded);
    assert!(history.entries(&file).is_empty());
}

#[test]
fn file_exceeding_max_size_is_skipped() {
    let temp = TempDir::new("max_size");
    let history = LocalHistory::new(
        temp.path.clone(),
        HistorySettings {
            max_file_kb: 1,
            ..Default::default()
        },
    );
    let file = temp.path.join("file.rs");
    let large_text = "x".repeat(1025);
    let recorded = history
        .record(&file, &large_text, Origin::Save, 100)
        .unwrap();
    assert!(!recorded);
    assert!(history.entries(&file).is_empty());

    let allowed_text = "x".repeat(1024);
    let recorded_ok = history
        .record(&file, &allowed_text, Origin::Save, 100)
        .unwrap();
    assert!(recorded_ok);
    assert_eq!(history.entries(&file).len(), 1);
}

#[test]
fn saves_within_merge_window_replace_entry_and_delete_old_blob() {
    let temp = TempDir::new("merge_window");
    let history = LocalHistory::new(
        temp.path.clone(),
        HistorySettings {
            merge_seconds: 10,
            ..Default::default()
        },
    );
    let file = temp.path.join("doc.rs");
    let text_a = "first save content";
    let text_b = "second save content";

    let recorded_a = history.record(&file, text_a, Origin::Save, 100).unwrap();
    assert!(recorded_a);
    let entries_first = history.entries(&file);
    assert_eq!(entries_first.len(), 1);
    let hash_a = entries_first[0].hash.clone();

    let recorded_b = history.record(&file, text_b, Origin::Save, 105).unwrap();
    assert!(recorded_b);

    let entries = history.entries(&file);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].time, 105);
    assert_eq!(entries[0].origin, Origin::Save);
    assert_eq!(
        history.content(&file, &entries[0]),
        Some(text_b.to_string())
    );

    let dir_hash = super::sha256_hex(b"doc.rs");
    let blob_a = temp
        .path
        .join(".crawrad")
        .join("history")
        .join(dir_hash)
        .join(format!("{hash_a}.txt"));
    assert!(!blob_a.exists());
}

#[test]
fn merge_seconds_zero_never_merges() {
    let temp = TempDir::new("merge_zero");
    let history = LocalHistory::new(
        temp.path.clone(),
        HistorySettings {
            merge_seconds: 0,
            ..Default::default()
        },
    );
    let file = temp.path.join("doc.rs");
    let recorded_a = history.record(&file, "first", Origin::Save, 100).unwrap();
    let recorded_b = history.record(&file, "second", Origin::Save, 105).unwrap();
    assert!(recorded_a);
    assert!(recorded_b);
    let entries = history.entries(&file);
    assert_eq!(entries.len(), 2);
}

#[test]
fn different_origins_within_merge_window_do_not_merge() {
    let temp = TempDir::new("merge_diff_origins");
    let history = LocalHistory::new(
        temp.path.clone(),
        HistorySettings {
            merge_seconds: 10,
            ..Default::default()
        },
    );
    let file = temp.path.join("doc.rs");
    let recorded_a = history
        .record(&file, "save text", Origin::Save, 100)
        .unwrap();
    let recorded_b = history
        .record(&file, "ext text", Origin::External, 102)
        .unwrap();
    assert!(recorded_a);
    assert!(recorded_b);
    let entries = history.entries(&file);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].origin, Origin::Save);
    assert_eq!(entries[1].origin, Origin::External);
}
