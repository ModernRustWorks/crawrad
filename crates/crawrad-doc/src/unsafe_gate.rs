//! **The gate that keeps `unsafe` in one crate** — [ADR 0009], finally armed.
//!
//! # Why it was missing for three weeks
//!
//! ADR 0009 decided on 31/08/2026 that *"every unsafe line of CrawRAD lives in
//! the FFI adapter, and the compiler guarantees the rest"*, and the mechanism
//! it chose is `#![forbid(unsafe_code)]` on every other crate root. The
//! mechanism is real — it is a compile error, not a convention — but **nothing
//! ever checked that a new crate root got the line**.
//!
//! That hole was measured on 17/09/2026, the day `crawrad-os` was born: the
//! makepad shell has a second crate root at `src/bin/crawrad-makepad.rs`, it
//! never declared the attribute, and it had been the IDE's main binary for
//! days. The rule was one `unsafe` block away from being silently over, in the
//! biggest binary in the workspace.
//!
//! # What it checks, and why the check is textual
//!
//! Every crate root under `crates/` — `src/lib.rs`, `src/main.rs` and every
//! `src/bin/*.rs` — carries `#![forbid(unsafe_code)]` on a line of its own.
//! Only [`THE_FFI_ADAPTER`] is excused.
//!
//! Textual, and not a `cargo` feature, because there is nothing else to read:
//! `forbid` has no manifest form, the attribute is the only statement of the
//! rule, and a missing attribute produces no diagnostic of any kind — the
//! crate simply compiles `unsafe` from then on. Reading the first lines of a
//! file is the whole check, and it is exact: the attribute has one spelling.
//!
//! # The exception is a list of one, on purpose
//!
//! A second name here means a second crate that may corrupt memory, and it
//! costs whoever adds it the sentence explaining why the question could not be
//! asked one layer up. That sentence is the actual gate; this file only makes
//! it unavoidable.
//!

use std::path::{Path, PathBuf};

/// The one crate allowed to say `unsafe`.
///
/// It is `crawrad-os`, and its own module header carries the measurement that
/// earned it the exception: no layer above it can be asked where the Windows
/// taskbar ends. See ADR 0009 and `crates/crawrad-os/src/lib.rs`.
pub(super) const THE_FFI_ADAPTER: &str = "crawrad-os";

/// The attribute, spelled the one way `rustc` accepts it as an inner attribute.
const THE_ATTRIBUTE: &str = "#![forbid(unsafe_code)]";

/// A crate root that does not forbid `unsafe`, and is not the FFI adapter.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Unguarded {
    /// The path from `crates/`, with forward slashes.
    pub path: String,
}

/// Every crate root under `crates/` missing the attribute.
///
/// A crate root is `src/lib.rs`, `src/main.rs` or any `src/bin/*.rs` — the
/// three shapes Cargo compiles as a separate crate, and therefore the three
/// places an inner attribute has to be repeated. A module declared with
/// `#[path]` is NOT a root and inherits the attribute from the root that
/// declares it, which is why nothing deeper is read.
pub(super) fn unguarded_crate_roots(crates_dir: &Path) -> Vec<Unguarded> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(crates_dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let crate_dir = entry.path();
        if !crate_dir.is_dir() {
            continue;
        }
        let name = crate_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        if name == THE_FFI_ADAPTER {
            continue;
        }
        for root in crate_roots_of(&crate_dir) {
            let Ok(source) = std::fs::read_to_string(&root) else {
                continue;
            };
            if !forbids_unsafe(&source) {
                found.push(Unguarded {
                    path: short_path(&root, crates_dir),
                });
            }
        }
    }
    found.sort();
    found
}

/// The crate roots of one crate directory.
fn crate_roots_of(crate_dir: &Path) -> Vec<PathBuf> {
    let src = crate_dir.join("src");
    let mut roots = Vec::new();
    for name in ["lib.rs", "main.rs"] {
        let p = src.join(name);
        if p.is_file() {
            roots.push(p);
        }
    }
    if let Ok(entries) = std::fs::read_dir(src.join("bin")) {
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x == "rs") {
                roots.push(p);
            }
        }
    }
    roots.sort();
    roots
}

/// Does this source carry the attribute on a line of its own?
///
/// On a line of its own, and that is the whole precision of this gate: the
/// exact same characters appear inside prose in `crawrad-ide/src/main.rs`,
/// which spends a paragraph explaining what the attribute does and does not
/// stop. A `contains` over the file would call that paragraph a guarantee.
fn forbids_unsafe(source: &str) -> bool {
    source.lines().any(|l| l.trim() == THE_ATTRIBUTE)
}

/// The path from `crates/`, with forward slashes so the message reads the same
/// on every platform.
fn short_path(file: &Path, crates_dir: &Path) -> String {
    file.strip_prefix(crates_dir)
        .unwrap_or(file)
        .display()
        .to_string()
        .replace('\\', "/")
}

#[cfg(test)]
#[path = "test/unsafe_gate/test.rs"]
mod tests;
