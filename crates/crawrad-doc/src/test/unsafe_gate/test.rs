//! The gate itself, plus the proof it can go red.

use super::{THE_FFI_ADAPTER, Unguarded, unguarded_crate_roots};
use std::path::{Path, PathBuf};

/// The `crates/` folder — the same walk the sibling gates do, by path and not
/// by dependency. Reading a file's text creates no edge in the graph.
fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// **Every crate root in the workspace forbids `unsafe`, except the adapter.**
///
/// This is ADR 0009 as a test. Until 17/09/2026 it was ADR 0009 as a promise,
/// and the promise was already broken: `crawrad-ide/src/bin/crawrad-makepad.rs`
/// had no attribute.
#[test]
fn only_the_ffi_adapter_may_say_unsafe() {
    let found = unguarded_crate_roots(&crates_dir());
    assert!(
        found.is_empty(),
        "these crate roots do not forbid `unsafe`:\n  {}\n\n\
         ADR 0009 says every crate but `{THE_FFI_ADAPTER}` declares \
         `#![forbid(unsafe_code)]` on a line of its own. Add it, or — if the \
         code really has to call the operating system — move the call into \
         `{THE_FFI_ADAPTER}` and write down there why no layer above it can \
         answer the question.",
        found
            .iter()
            .map(|u| u.path.clone())
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

/// **The adapter is excused, and it is the only one.**
///
/// The mutation this refuses: excusing a crate by widening the rule instead of
/// by naming it. If the skip ever became "any crate whose name starts with
/// `crawrad-o`", this test would still pass — so it also asserts the exception is
/// a whole name, not a prefix.
#[test]
fn the_exception_is_one_whole_name() {
    assert_eq!(THE_FFI_ADAPTER, "crawrad-os");
    assert!(
        crates_dir()
            .join(THE_FFI_ADAPTER)
            .join("src/lib.rs")
            .is_file(),
        "the exception names a crate that does not exist — dead licence"
    );
}

/// **A crate root without the attribute is really caught.**
///
/// Written to a temporary tree, because pointing the gate at the workspace can
/// only prove the green. A gate that has never been seen red is a gate nobody
/// has measured.
#[test]
fn a_root_without_the_attribute_is_caught() {
    let base = std::env::temp_dir().join(format!(
        "crawrad-unsafe-gate-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&base);

    let guarded = base.join("crawrad-guarded/src");
    let bare = base.join("crawrad-bare/src");
    let excused = base.join(THE_FFI_ADAPTER).join("src");
    for d in [&guarded, &bare, &excused] {
        std::fs::create_dir_all(d).expect("temporary tree");
    }
    std::fs::write(
        guarded.join("lib.rs"),
        "//! ok\n#![forbid(unsafe_code)]\npub fn f() {}\n",
    )
    .unwrap();
    std::fs::write(bare.join("lib.rs"), "//! nothing forbidden here\n").unwrap();
    std::fs::write(excused.join("lib.rs"), "//! the adapter\n").unwrap();
    // A second root in the SAME crate — the exact shape that was broken in
    // `crawrad-ide`, where `src/main.rs` had the attribute and `src/bin/*.rs`
    // did not.
    std::fs::create_dir_all(guarded.join("bin")).unwrap();
    std::fs::write(guarded.join("bin/tool.rs"), "fn main() {}\n").unwrap();

    let found = unguarded_crate_roots(&base);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(
        found,
        vec![
            Unguarded {
                path: "crawrad-bare/src/lib.rs".to_string()
            },
            Unguarded {
                path: "crawrad-guarded/src/bin/tool.rs".to_string()
            },
        ],
        "the bare crate AND the unguarded second root have to show up, and the \
         adapter must not"
    );
}

/// **The attribute quoted inside prose is not the attribute.**
///
/// `crawrad-ide/src/main.rs` really does spend a paragraph on
/// `#![forbid(unsafe_code)]`, and a `contains` over the file would read that
/// paragraph as a guarantee. This is the file shape that would fool it.
#[test]
fn the_attribute_named_in_a_comment_does_not_count() {
    let base = std::env::temp_dir().join(format!(
        "crawrad-unsafe-prose-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&base);
    let src = base.join("crawrad-talker/src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("lib.rs"),
        "//! This crate explains what `#![forbid(unsafe_code)]` means.\n\
         //! It does not declare it.\npub fn f() {}\n",
    )
    .unwrap();

    let found = unguarded_crate_roots(&base);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(
        found,
        vec![Unguarded {
            path: "crawrad-talker/src/lib.rs".to_string()
        }],
        "talking about the attribute is not declaring it"
    );
}
