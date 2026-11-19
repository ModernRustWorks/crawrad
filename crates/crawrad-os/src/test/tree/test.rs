//! A real tree: `cmd` running two `ping`s is three processes. Killing the
//! root alone leaves the other two running — the orphans — and
//! ending the tree takes all of them. `cmd` is Windows: so is every test here.
//!
//! **The children are known to exist before the root dies**. The
//! test used to wait for "three processes in the tree" and kill the root:
//! but the console host Windows starts for `cmd` counts too, so three could
//! be reached before `cmd` had launched its children — it died first, and
//! there were no orphans to count (measured: 1 run in 20, `the orphans: 0`,
//! and no `ping` left running anywhere). Now `cmd` launches two `ping`s with
//! `start /b`, which returns only once the process exists, and then writes
//! a marker file; the test waits for the marker, not for a count.
#![cfg(windows)]

use super::ProcessTree;
use std::time::{Duration, Instant};

fn until(what: &str, mut ok: impl FnMut() -> bool) {
    let started = Instant::now();
    while !ok() {
        assert!(started.elapsed() < Duration::from_secs(10), "{what}");
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// `cmd` with two long `ping`s already started, and the marker that says so
/// (`cmd` itself then waits on a third `ping`, so it is alive when killed).
fn a_tree(name: &str) -> (std::process::Child, std::path::PathBuf) {
    let marker =
        std::env::temp_dir().join(format!("crawrad-os-tree-{name}-{}", std::process::id()));
    let _ = std::fs::remove_file(&marker);
    let line = format!(
        "start /b ping -n 60 127.0.0.1 > nul & start /b ping -n 60 127.0.0.1 > nul & echo up> \"{}\" & ping -n 60 127.0.0.1 > nul",
        marker.display()
    );
    // the line goes as it is: Rust's quoting (`\"`) is not cmd's
    use std::os::windows::process::CommandExt;
    let child = crate::process::no_console("cmd")
        .arg("/c")
        .raw_arg(&line)
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();
    (child, marker)
}

/// **Killing the root leaves its children; ending the tree does not.**
#[test]
fn ending_the_tree_ends_the_children_the_root_left() {
    let (mut root, marker) = a_tree("ending");
    let tree = ProcessTree::adopt(&root).unwrap();
    until("cmd has started both pings", || marker.exists());
    let _ = std::fs::remove_file(&marker);

    root.kill().unwrap();
    let _ = root.wait();
    std::thread::sleep(Duration::from_millis(300));
    assert!(tree.alive() >= 2, "the orphans: {}", tree.alive());

    tree.end();
    until("the tree is gone", || tree.alive() == 0);
}

/// **Dropping the tree ends it too** (kill on close): an IDE that dies
/// without a clean stop still takes the server's children with it.
#[test]
fn dropping_the_tree_ends_it() {
    let (root, marker) = a_tree("dropping");
    let tree = ProcessTree::adopt(&root).unwrap();
    until("cmd has started both pings", || marker.exists());
    let _ = std::fs::remove_file(&marker);
    let pid = root.id();
    drop(tree);
    let mut root = root;
    until("the root is gone", || root.try_wait().unwrap().is_some());
    assert_ne!(pid, 0);
}
