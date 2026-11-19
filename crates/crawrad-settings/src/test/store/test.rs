use super::*;
use std::path::Path;

fn temp_db_path(prefix: &str) -> PathBuf {
    let id = monotonic_timestamp();
    std::env::temp_dir().join(format!(
        "crawrad_test_{prefix}_{id}_{}.db",
        std::process::id()
    ))
}

#[test]
fn temp_file_set_and_get() {
    let path = temp_db_path("set_get");
    let mut store = SettingsStore::open(&path).expect("open store");

    assert_eq!(store.get("test-key"), None);
    store.set("test-key", "test-val").expect("set");
    assert_eq!(store.get("test-key"), Some("test-val".to_string()));

    // Overwrite existing key
    store.set("test-key", "updated-val").expect("update");
    assert_eq!(store.get("test-key"), Some("updated-val".to_string()));

    let _ = std::fs::remove_file(&path);
}

#[test]
fn set_many_in_single_transaction() {
    let path = temp_db_path("set_many");
    let mut store = SettingsStore::open(&path).expect("open store");

    let pairs = [("theme", "dark"), ("font_size", "14"), ("tab_size", "4")];
    store.set_many(&pairs).expect("set_many");

    assert_eq!(store.get("theme"), Some("dark".to_string()));
    assert_eq!(store.get("font_size"), Some("14".to_string()));
    assert_eq!(store.get("tab_size"), Some("4".to_string()));

    let all = store.all();
    assert_eq!(all.len(), 3);
    assert_eq!(all[0], ("font_size".to_string(), "14".to_string()));
    assert_eq!(all[1], ("tab_size".to_string(), "4".to_string()));
    assert_eq!(all[2], ("theme".to_string(), "dark".to_string()));

    let _ = std::fs::remove_file(&path);
}

#[test]
fn reopen_and_read_back() {
    let path = temp_db_path("reopen");
    {
        let mut store = SettingsStore::open(&path).expect("open store");
        store.set("retained_key", "persisted_value").expect("set");
        store
            .push_recent_folder(Path::new("/some/project"))
            .expect("push");
    }

    {
        let mut reopened = SettingsStore::open(&path).expect("reopen store");
        assert_eq!(
            reopened.get("retained_key"),
            Some("persisted_value".to_string())
        );
        let recents = reopened.recent_folders(10);
        assert_eq!(recents, vec![PathBuf::from("/some/project")]);
    }

    let _ = std::fs::remove_file(&path);
}

#[test]
fn recent_folders_order_and_forget() {
    let path = temp_db_path("recents");
    let mut store = SettingsStore::open(&path).expect("open store");

    let p1 = Path::new("/workspace/project_a");
    let p2 = Path::new("/workspace/project_b");
    let p3 = Path::new("/workspace/project_c");

    store.push_recent_folder(p1).expect("push p1");
    store.push_recent_folder(p2).expect("push p2");
    store.push_recent_folder(p3).expect("push p3");

    // Newest first: p3, p2, p1
    let recents = store.recent_folders(10);
    assert_eq!(
        recents,
        vec![p3.to_path_buf(), p2.to_path_buf(), p1.to_path_buf()]
    );

    // Re-pushing p1 bumps it to the top
    store.push_recent_folder(p1).expect("re-push p1");
    let recents = store.recent_folders(10);
    assert_eq!(
        recents,
        vec![p1.to_path_buf(), p3.to_path_buf(), p2.to_path_buf()]
    );

    // Limit check
    let recents_limited = store.recent_folders(2);
    assert_eq!(recents_limited, vec![p1.to_path_buf(), p3.to_path_buf()]);

    // Forget p3
    store.forget_recent_folder(p3).expect("forget p3");
    let recents_after = store.recent_folders(10);
    assert_eq!(recents_after, vec![p1.to_path_buf(), p2.to_path_buf()]);

    let _ = std::fs::remove_file(&path);
}

#[test]
fn opening_corrupt_file_returns_err_no_panic() {
    let path = temp_db_path("corrupt");
    std::fs::write(&path, b"not a sqlite database file at all").expect("write plain text");

    let result = SettingsStore::open(&path);
    assert!(result.is_err(), "opening invalid db must return Err");

    let _ = std::fs::remove_file(&path);
}

#[cfg(windows)]
#[test]
fn one_recent_per_folder_whatever_the_separators() {
    let path = temp_db_path("recents_separators");
    let mut store = SettingsStore::open(&path).expect("open store");
    store
        .push_recent_folder(Path::new("C:/work/my_app"))
        .expect("push");
    store
        .push_recent_folder(Path::new(r"C:\work\other"))
        .expect("push");
    store
        .push_recent_folder(Path::new(r"C:\work\my_app\"))
        .expect("push");
    let recents = store.recent_folders(10);
    assert_eq!(recents.len(), 2, "{recents:?}");
    assert_eq!(recents[0], Path::new(r"C:\work\my_app"));
    store
        .forget_recent_folder(Path::new("C:/work/other"))
        .expect("forget");
    assert_eq!(store.recent_folders(10).len(), 1);
    drop(store);
    let _ = std::fs::remove_file(&path);
}
