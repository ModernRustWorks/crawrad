//! The order the IDE looks for rust-analyzer in, on fake folders.

use std::path::{Path, PathBuf};

use super::{SERVER_DIR, ServerSource, locate_from, server_file_name};

/// A scratch folder with a fake server in each place that is asked for.
struct Places {
    root: PathBuf,
}

impl Places {
    fn new(name: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("crawrad-ra-locate-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        Self { root }
    }
    fn dir(&self, sub: &str) -> PathBuf {
        let d = self.root.join(sub);
        std::fs::create_dir_all(&d).unwrap();
        d
    }
    /// Puts a fake server file in `sub` and returns its path.
    fn server_in(&self, sub: &str) -> PathBuf {
        let path = self.dir(sub).join(server_file_name());
        std::fs::write(&path, b"fake").unwrap();
        path
    }
    fn ide(&self) -> PathBuf {
        self.dir("ide")
    }
    fn path_var(&self) -> Option<std::ffi::OsString> {
        Some(self.dir("system").into_os_string())
    }
    fn home(&self) -> Option<PathBuf> {
        Some(self.dir("home"))
    }
}

impl Drop for Places {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn bundled_sub() -> String {
    Path::new("ide")
        .join(SERVER_DIR)
        .to_string_lossy()
        .into_owned()
}

/// **The IDE's own wins over the machine's**, even when the PATH has one.
#[test]
fn the_ides_own_beats_the_path() {
    let p = Places::new("own");
    let own = p.server_in(&bundled_sub());
    p.server_in("system");
    let found = locate_from(None, Some(&p.ide()), p.path_var(), p.home()).unwrap();
    assert_eq!(found.path, own);
    assert_eq!(found.source, ServerSource::Bundled);
}

/// **The override wins over everything.**
#[test]
fn the_variable_beats_the_ides_own() {
    let p = Places::new("env");
    let named = p.server_in("named");
    p.server_in(&bundled_sub());
    let found = locate_from(Some(named.clone()), Some(&p.ide()), p.path_var(), p.home()).unwrap();
    assert_eq!((found.path, found.source), (named, ServerSource::Override));
}

/// **Without the IDE's own, the PATH is the last resort — and it says so.**
#[test]
fn the_path_is_the_last_resort_and_reported_as_system() {
    let p = Places::new("path");
    let system = p.server_in("system");
    let found = locate_from(None, Some(&p.ide()), p.path_var(), p.home()).unwrap();
    assert_eq!((found.path, found.source), (system, ServerSource::System));
}

/// **`~/.cargo/bin` after the PATH, also System.**
#[test]
fn cargo_bin_after_the_path() {
    let p = Places::new("cargo");
    let cargo = p.server_in(
        &Path::new("home")
            .join(".cargo")
            .join("bin")
            .to_string_lossy(),
    );
    let found = locate_from(None, Some(&p.ide()), p.path_var(), p.home()).unwrap();
    assert_eq!((found.path, found.source), (cargo, ServerSource::System));
}

/// **None anywhere: the error names the IDE's folder first**, so the user
/// sees where the server should have been.
#[test]
fn missing_names_where_it_looked() {
    let p = Places::new("none");
    let missing = locate_from(
        Some(p.root.join("nowhere.exe")),
        Some(&p.ide()),
        p.path_var(),
        p.home(),
    )
    .unwrap_err();
    assert_eq!(missing.looked_at[0], p.root.join("nowhere.exe"));
    assert_eq!(
        missing.looked_at[1],
        p.ide().join(SERVER_DIR).join(server_file_name())
    );
    let text = missing.to_string();
    assert!(text.contains("was not found"), "{text}");
}

/// **The bar warns about the machine's copy, and only about it.**
#[test]
fn only_the_system_copy_is_marked_on_the_bar() {
    assert_eq!(
        ServerSource::System.status_name(),
        "rust-analyzer (system copy)"
    );
    assert_eq!(ServerSource::Bundled.status_name(), "rust-analyzer");
    assert_eq!(ServerSource::Override.status_name(), "rust-analyzer");
}
