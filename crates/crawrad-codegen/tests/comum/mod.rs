//! The rule that BOTH gates share — which diverged while it was two copies.
//!
//! `gate_fase3` and `gate_sem_avisos` assemble a draft inside `target/`
//! and check, before invoking `cargo`, whether it is still intact. The
//! artifact set is DIFFERENT between the two — there it is three (the mock
//! repository, the group root, and the member), here it is two — and that difference is
//! legitimate, because each gate writes what it writes.
//!
//! What could **not** diverge was the criterion of "is this manifest mine?": it
//! is the exact same question in both places. And it diverged. `gate_fase3` replaced
//! `contains("[workspace]")` with byte-for-byte equality, proved the change via
//! mutation testing, and documented the rationale — while its twin retained `contains` in the same
//! commit. The adjacent comment even predicted this exact defect ("being
//! the same it is TWO copies — so each copy needs its own
//! gate, or one of them will drift on its own again") and it occurred anyway,
//! because **each copy's gate only looks at its own copy**. Two correct guards,
//! each verified in its own file, with neither able to notice that the other
//! changed.
//!
//! Hence this module: the rule that must not diverge stops being two copies and
//! becomes a single function, eliminating the problem instead of adding another watcher.
//! This follows project rule 4 — one implementation per concept.

use std::path::Path;

/// Is the workspace manifest on disk, byte for byte, **the one** the gate writes?
///
/// Read and compare, not `.exists()`: a zero-byte file exists and cargo
/// cannot open it. And compare by EQUALITY, not `contains("[workspace]")`,
/// because the second question is the wrong one — it answers "is there A workspace
/// manifest here?" when the guard needs to know "is THE manifest here?".
///
/// **The state that only equality catches, measured:** with the manifest reduced to
/// `"[workspace]\nmembers = []"`, `contains` answers **intact**. Nothing
/// rewrites the file, `cargo new` tries to register the member in a `members` list
/// that does not list it, and the gate dies — on every subsequent run, identically,
/// because a guard that judges intact never fixes anything. That is the disease this function
/// exists to cure.
///
/// And this state does NOT come from cache pruning: it comes from someone editing the manifest
/// constant without changing the draft path — which is exactly what
/// `d67f027` did. The disk retains the manifest from the previous file version, and
/// only the equality check catches it.
///
/// **Costs no extra run**, and this was measured before switching: `cargo new`
/// and `cargo add` do not touch any workspace manifest (identical md5
/// before and after, 2026-09-07), so the newly assembled draft returns `true`
/// on the next run and the group's `target/` stays warm.
///
/// Both gates prove this function via mutation testing, each with its own table:
/// changing `==` to `contains` drops the `group root with DIFFERENT content`
/// line in both, and the `repository manifest with DIFFERENT content` line in
/// `gate_fase3`.
pub fn manifesto_intocado(pasta: &Path, esperado: &str) -> bool {
    std::fs::read_to_string(pasta.join("Cargo.toml")).is_ok_and(|m| m == esperado)
}
