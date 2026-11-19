//! Where the adapter is looked for — and where it is not.

use std::path::PathBuf;

use crate::adapter::{ADAPTER_DIR, adapter_file_name, locate_from};

fn scratch(name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("crawrad-dap-adapter-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn the_adapter_beside_the_ide_is_the_one_used() {
    let ide = scratch("bundled");
    let bundled = ide.join(ADAPTER_DIR).join(adapter_file_name());
    std::fs::create_dir_all(bundled.parent().unwrap()).unwrap();
    std::fs::write(&bundled, b"").unwrap();
    assert_eq!(locate_from(None, Some(&ide)), Ok(bundled));
}

#[test]
fn the_override_wins_when_it_exists_and_is_skipped_when_it_does_not() {
    let ide = scratch("override");
    let own = ide.join("my-lldb-dap.exe");
    std::fs::write(&own, b"").unwrap();
    assert_eq!(locate_from(Some(own.clone()), Some(&ide)), Ok(own));
    let bundled = ide.join(ADAPTER_DIR).join(adapter_file_name());
    std::fs::create_dir_all(bundled.parent().unwrap()).unwrap();
    std::fs::write(&bundled, b"").unwrap();
    let gone = ide.join("gone.exe");
    assert_eq!(locate_from(Some(gone), Some(&ide)), Ok(bundled));
}

#[test]
fn nothing_found_says_where_it_looked_and_never_tries_the_path() {
    let ide = scratch("missing");
    let missing = locate_from(None, Some(&ide)).unwrap_err();
    let expected = ide.join(ADAPTER_DIR).join(adapter_file_name());
    assert_eq!(missing.looked_at, vec![expected.clone()]);
    let text = missing.to_string();
    assert!(text.starts_with("The debugger that comes with the IDE is not installed"));
    assert!(text.contains(&expected.display().to_string()), "{text}");
    // An `lldb-dap` on the PATH is not the IDE's: it is never looked for.
    assert!(!text.contains("PATH"));
    // `CRAWRAD_LLDB_DAP=` (set but empty) is no place: it is not listed.
    let empty = locate_from(Some(PathBuf::new()), Some(&ide)).unwrap_err();
    assert_eq!(empty.looked_at, vec![expected]);
}
