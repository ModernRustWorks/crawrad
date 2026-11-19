//! **No generator target translates a name.** What the user types is what the
//! application compiles.
//!
//! The owner's law, 16/09/2026:
//!
//! > *"Here it is RUST, not the CLASSIC RAD IDE. The classic IDE here is only the idea. I do not
//! > want to help the classic IDE dev come and write as if he were in the classic IDE with
//! > the IDE coping with it. Take it out — here it is Rust, IT HAS TO BE
//! > WRITTEN THE WAY RUST ALLOWS."*
//!
//! # The defect this file freezes
//!
//! A component's `Name` becomes a field (`pub btn_ok: Button`) and a method in
//! the generated project. Until that date the makepad target ran every name
//! through a case conversion before emitting it — **23 call sites** — and the
//! egui target ran none. So a component named `btnOK` came out as
//! `self.btn_ok` on one target and `self.btnOK` on the other: the name the
//! Inspector showed was not the name the code had, and the two targets
//! disagreed about the name of the same field.
//!
//! The asymmetry was the tell, and egui was the side that was right. This gate
//! locks egui where it already is and charges makepad for the change.
//!
//! # Where the refusal went instead
//!
//! Taking the conversion out without replacing it would let `non_snake_case`
//! reach the user's project — in a file the user did not write and cannot fix,
//! since the `.designer.rs` is rewritten whole on the next generation. Silencing
//! it with `#[allow]` is the one answer this house does not take (see
//! `test/no_allow`), and it is the one that already hid eighteen product
//! defects here.
//!
//! So the refusal moved to the **entrance**: `Doc::apply` rejects a `Name` that
//! is not a Rust identifier in snake_case, and the rejection carries the right
//! form (`btnOK` → *use `btn_ok`*). The conversion logic survives as
//! `crawrad_ir::snake_case_hint` — no longer a silent fixer, now the advice that
//! comes with the refusal.
//!
//! # The one case conversion that stays, and why it is not translation
//!
//! `nome.to_uppercase()` in the egui `ComboBox`, `ListBox` and `TreeView`, and
//! only there. It does not name a field: it names the **const** that holds the
//! item list, and `SCREAMING_SNAKE` for a const is Rust's own convention — the
//! opposite of dressing a Rust name up as a classic RAD one. The gate allows it only
//! next to the `_ITEMS` suffix that identifies it, so `name.to_uppercase()`
//! anywhere else is still caught.

use std::path::{Path, PathBuf};

/// Case conversions that **lower** a name. Every one of them is a candidate to
/// turn `Button1` into `button1` behind the user's back, which is the whole
/// thing that was removed.
///
/// `para_snake_case` is in the list even though the function no longer exists
/// in this workspace: a name in a gate costs one line and keeps the deleted
/// thing from walking back in under its old name. It lived **twice**, character
/// for character, in `crawrad-codegen/src/lib.rs` and `crawrad-registry/src/lib.rs`.
const LOWERING_CALLS: &[&str] = &[
    ".to_lowercase()",
    ".to_ascii_lowercase()",
    "para_snake_case(",
    "to_snake_case(",
    // The advice the refusal gives. It is correct where a human reads it and
    // wrong where code emits it: a target calling it is translating again.
    "snake_case_hint(",
];

/// Case conversions that **raise** a name. Allowed only where they build the
/// name of a const — see the module doc.
const RAISING_CALLS: &[&str] = &[".to_uppercase()", ".to_ascii_uppercase()"];

/// What makes a raising conversion legitimate: it is naming a const, and the
/// const's suffix is right there.
const CONST_MARKER: &str = "_ITEMS";

/// How far from the call the marker may be. Six lines is what the three
/// legitimate sites need today — the widest of them puts the call in the
/// argument list of a `format!` whose literal opens six lines above.
const MARKER_WINDOW: usize = 6;

fn targets_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/targets")
}

/// Every `.rs` under `src/targets`, tests included.
///
/// Tests are **not** excluded, and that is deliberate: a fixture that has to
/// convert a name to match what the generator emits is the same defect one
/// layer up, and it is how the removal would be quietly undone.
fn target_sources(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(target_sources(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    out.sort();
    out
}

fn relative(path: &Path) -> String {
    path.strip_prefix(Path::new(env!("CARGO_MANIFEST_DIR")))
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}

#[test]
fn no_target_converts_the_case_of_a_name() {
    let dir = targets_dir();
    let files = target_sources(&dir);
    assert!(
        files.len() > 10,
        "the sweep found {} files under {} — it is measuring nothing",
        files.len(),
        dir.display()
    );

    let mut complaints = Vec::new();
    let mut scanned = 0usize;
    for file in &files {
        let text = std::fs::read_to_string(file).expect("a source file of this crate");
        let lines: Vec<&str> = text.lines().collect();
        scanned += lines.len();
        for (i, line) in lines.iter().enumerate() {
            for needle in LOWERING_CALLS {
                if line.contains(needle) {
                    complaints.push(format!(
                        "{}:{}: `{needle}` — a target does not fix a name, it emits the one the user wrote\n    {}",
                        relative(file),
                        i + 1,
                        line.trim()
                    ));
                }
            }
            for needle in RAISING_CALLS {
                if !line.contains(needle) {
                    continue;
                }
                let from = i.saturating_sub(MARKER_WINDOW);
                let to = (i + MARKER_WINDOW + 1).min(lines.len());
                if lines[from..to].iter().any(|l| l.contains(CONST_MARKER)) {
                    continue;
                }
                complaints.push(format!(
                    "{}:{}: `{needle}` away from `{CONST_MARKER}` — raising case is for the name of a const, not for a field\n    {}",
                    relative(file),
                    i + 1,
                    line.trim()
                ));
            }
        }
    }

    assert!(
        scanned > 1000,
        "only {scanned} lines read: the sweep is not reaching the targets"
    );
    assert!(
        complaints.is_empty(),
        "a generator target is translating a name ({} complaint(s)):\n{}\n\n\
         The rule, from the owner (16/09/2026): the name the user sees in the \
         Inspector is the name his code has. A name that is not snake_case is \
         refused at the entrance by `Doc::apply`, with the right form \
         suggested — it is not fixed underneath.",
        complaints.len(),
        complaints.join("\n")
    );
}
