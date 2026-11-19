//! The class dispatch gate: **each component answers for itself; nobody
//! answers for all of them.**
//!
//! The owner's law, 15/09/2026:
//!
//! > *"That file full of component `if`s, when the component is the one
//! > responsible for its own feature — providing its own code is HIS job."*
//!
//! And, earlier, pointing at the original defect:
//!
//! > *"`canvas.rs` is handling everything for everyone… it needed a `.rs`
//! > where that code belongs to the `Edit`, so IT resolves and hands the
//! > result back to the `canvas`."*
//!
//! # The disease this module cures
//!
//! The rule **was already written** — `CONTRIBUTING.md`, *Makepad component rules*
//! forbids, in so many words, *"monolithic `match` blocks or centralized
//! `if class == "…"` chains in `form.rs` or `actions.rs`"* — and it **became a
//! REJECT item in both reviewers** on the same day. It escaped anyway: one
//! workstream tied DSL ids to class names in a 14-entry table, with the twin
//! list repeated in the DSL right beside it. The two diverged immediately —
//! `ActionList`, `ImageList` and `Timer` were in one and not in the other,
//! three dead rows on screen — and **the compiler had nothing to complain
//! about**.
//!
//! The reviewer caught it because the rule had just entered its definition.
//! A reviewer only runs when someone fires it; **a guard runs always**. This
//! module is the same rule in the shape of a test, charged at `cargo test`, in
//! the very commit that writes the dispatcher.
//!
//! # The ruler, and how it was measured
//!
//! The idea is easy to state: **a file that decides the behaviour of many
//! catalog classes is a central dispatcher**. Both halves of the ruler — *which
//! classes* and *what counts as deciding* — were chosen with the tree measured
//! in hand, not guessed.
//!
//! **Which classes.** They are not written here: they come from
//! `crawrad_registry::Registry::builtin()`, asked at every run. A new class in the
//! catalog starts being charged on its own, and this file never finds out — the
//! same design as the `$TARGETS` of `target_command_gate`, with one
//! difference in our favour: there the source is a `.ps1` that can only be read
//! as text, while here the catalog **is a Rust API of this workspace**. Asking
//! the API does not go stale; copying the list does. The edge is a
//! `[dev-dependencies]` one, it does not enter the graph this house publishes,
//! and there is no cycle: `crawrad-registry` depends on `crawrad-ir` and nothing else.
//!
//! **What counts as deciding.** Only a class name sitting where it **decides
//! the path**: a `match` arm, an operand of `==`/`!=`, a pattern alternative
//! (`"A" | "B" =>`) and `matches!(…)`, which is the one-line `match`. That
//! choice is the heart of the ruler, and the measurement explains why: the tree
//! has test files quoting **16, 18 and 19** class names as literals —
//! `Node::new("btn1", "Button")` and siblings — and **none** of them is a
//! dispatcher. They are fixtures, and a fixture decides nothing: it passes the
//! name as an argument. Counting mentions would accuse the whole tree on day
//! one; counting decisions separates the two with precision, and the 15/09/2026
//! measurement left not a single false positive.
//!
//! The number lives in [`THRESHOLD`], with the distribution that justifies it.
//!
//! # What it does NOT charge, said in full
//!
//! 1. **`crawrad-registry`** ([`CATALOG_CRATE`]). It is the catalog: knowing every
//!    class is its job, and what it holds is a **descriptor**, not behaviour.
//! 2. **The per-component file** — with no written exclusion at all, which is
//!    the pretty part. The 77 files under `componentes/` and `components/`
//!    decide, at most, about **1** class: their own. The threshold already
//!    absolves them by construction, and not opening a hole named after those
//!    folders is deliberate — a hole by path becomes the place where the next
//!    dispatcher will live. A test charges that this stays true.
//! 3. **The quotation in a comment, a doc or a string.** Talking about the
//!    forbidden thing is not doing it. That is the `portao_do_lint_silenciado`
//!    technique, and this gate borrows its lexer instead of writing a second
//!    one — an anti-duplication gate born duplicating would be ironic twice.
//! 4. **A `match` over a class enum.** A `match ComponentClass::Button => …`
//!    passes. That is not an oversight: in the `.crawform` and in
//!    [`crawrad_ir::Node`] the class **is a string**, and every real dispatch goes
//!    through it; telling an `enum` variant apart from any other variant
//!    lexically would need types, not a lexer. Whoever converts to an `enum`
//!    before dispatching is dodging the gate **on purpose**, which is a
//!    different conversation.
//! 5. **The data table** — `("item_btn", "Button")` instead of a `match`.
//!    Lexically it is identical to a test fixture, and the ruler cannot tell
//!    them apart. It is this guard's measured boundary, and it is written here
//!    so that nobody discovers it thinking it is a hidden hole.
//! 6. **Whether the dispatch that remains is well written.** The gate promises
//!    it is not central; judging its content is the reviewer's job.

use crate::silenced_lint_gate::{fim_da_string, fim_da_string_crua, fim_do_caractere};

/// How many distinct classes one file may decide before being a dispatcher.
///
/// **Eight, out of a catalog of twenty**, and the number is measured. The
/// distribution of distinct classes **decided** per file, on 15/09/2026, over
/// the whole tree (`crates/`, 21 files with at least one point):
///
/// ```text
/// 19  crawrad-runtime-egui/src/lib.rs          <- declared debt
/// 16  crawrad-codegen/src/targets/egui/form.rs <- declared debt
/// 16  crawrad-registry/src/classes.rs          <- the catalog, out of the sweep
/// ---------------------------------------------- the empty band: 7 to 15
///  6  crawrad-codegen/src/targets/egui/layout.rs   {the 4 non-visuals + PageControl/TabSheet}
///  6  crawrad-ide/src/main.rs                    {ActionList, ImageList, TreeView, ComboBox, …}
///  4  crawrad-codegen/src/targets/makepad/layout.rs
///  4  crawrad-actions/src/lib.rs, crawrad-ide/src/tree.rs, crawrad-runtime-egui/src/varredura.rs
///  2  four files, all of them `PageControl` + `TabSheet`
///  1  seven files, among them the ones under `components/`
/// ```
///
/// The cut lands **in the middle of an empty band nine numbers wide** (7 to
/// 15), and that is where the slack on both sides comes from: today's largest
/// legitimate file has two classes to spare, and the smallest dispatcher has
/// eight. It is not a round number picked by taste — it is the only place where
/// the line touches nobody.
///
/// And the meaning matches the shape: **below eight, what the measurement shows
/// are families**, not catalogs. `PageControl` and `TabSheet` are a single
/// concept (the strip and the sheet do not exist apart, just like in a classic RAD IDE);
/// `ActionList`, `Action`, `ImageList` and `Timer` are the non-visuals, and the
/// question the four of them answer together is *"does this draw?"*. Eight out
/// of twenty is no family at all: it is the catalog's behaviour written outside
/// the catalog.
///
/// **This number is meant to GO DOWN.** It portrays a tree that carries debt,
/// it is not a goal: every dispatcher turned into per-component files tightens
/// what the next one may do. Lowering it after paying the debt is using the
/// guard exactly as it was designed.
pub(super) const THRESHOLD: usize = 8;

/// The crate that **may** know every class: the catalog.
///
/// Knowing the whole catalog is its job — the target `CLASSES` lists and the
/// `as_str()` in `classes.rs` decide over the 16 classes of the `enum`, and so
/// they should. What it holds is descriptors: name, property, event, default.
/// **Component behaviour does not live here**, and that boundary is what makes
/// this exclusion cheap.
pub(super) const CATALOG_CRATE: &str = "crawrad-registry/";

/// The central dispatchers the tree **has today**, with the reason and issue.
///
/// A declared exception is visible debt; a hidden one is a licence. The ones
/// below were born with this guard, on 15/09/2026, and neither was fixed in
/// this round on purpose: the guard had to be born **green**, with the debt in
/// the diff, instead of being born red and switched off in the first rush.
///
/// An exception matching no file is charged as an error by the test, and so is
/// an exception matching a file **below the threshold**: both rot silently, and
/// the second is worse — it covers a file that has already been fixed and keeps
/// authorizing the next dispatcher to be born right there.
pub(super) const EXCEPTIONS: &[(&str, &str)] = &[(
    "crawrad-runtime-egui/src/lib.rs",
    "19 classes, 48 points, the largest being the drawing \
         `match c.class.as_str()` at :2484. This is issue #61 — `the interface \
         guesses capability from the class name in five places, and one of \
         them already bit us`, which already measured the defect of the \
         `ListBox` opening in tree mode. The fix is the same path \
         `crawrad-runtime-makepad` took: 19 per-class files under \
         `src/componentes/` and ZERO dispatch points in the whole crate.",
)];

/// The catalog classes, asked of `crawrad-registry`.
///
/// Twenty on 15/09/2026, and the list is not ours: it is `todos_builtin()`'s.
/// Note it is larger than what the targets generate (16) — `Form`,
/// `ActionList`, `Action` and `ImageList` exist only in the `Registry`. Asking
/// the `Registry` and not the `enum` is what makes this guard cover the four
/// the `enum` forgot.
pub(super) fn catalog_classes() -> Vec<&'static str> {
    crawrad_registry::Registry::builtin()
        .all()
        .map(|d| d.class)
        .collect()
}

/// The source with **comments blanked** and **strings blanked**, except those
/// whose content is **exactly** a class name.
///
/// It is `portao_do_lint_silenciado`'s `mascarar` with one difference: there
/// every string vanishes, because there the complaint is about code; here the
/// accused text **is** a literal (`c.class.as_str() == "Button"`), and erasing
/// it would erase the complaint. The rule for what stays is the WHOLE literal
/// being a class name — and that rule is what absolves, with no written
/// exception at all, a chunk of code sitting inside a string: in
/// `r#"match c { "Button" => … }"#` the literal is the entire block, its
/// content is not `Button`, and the block vanishes with all the others.
///
/// What stays is normalized to the plain form `"Class"` and padded with spaces
/// up to the original literal's length — so `r#"Button"#` and `"Button"` reach
/// the seeker alike, and no column moves. The `\n` is always preserved, so that
/// the complaint's line is the file's line.
pub(super) fn code_with_class_literals(source: &str, classes: &[&str]) -> String {
    let cs: Vec<char> = source.chars().collect();
    let n = cs.len();
    let mut out = String::with_capacity(source.len());
    let mut i = 0usize;

    while i < n {
        let c = cs[i];

        if c == '/' && cs.get(i + 1) == Some(&'/') {
            let mut j = i;
            while j < n && cs[j] != '\n' {
                j += 1;
            }
            blank(&cs, i, j, &mut out);
            i = j;
            continue;
        }

        if c == '/' && cs.get(i + 1) == Some(&'*') {
            let mut j = i + 2;
            let mut depth = 1usize;
            while j < n && depth > 0 {
                if cs[j] == '/' && cs.get(j + 1) == Some(&'*') {
                    depth += 1;
                    j += 2;
                } else if cs[j] == '*' && cs.get(j + 1) == Some(&'/') {
                    depth -= 1;
                    j += 2;
                } else {
                    j += 1;
                }
            }
            let j = j.min(n);
            blank(&cs, i, j, &mut out);
            i = j;
            continue;
        }

        let end = fim_da_string_crua(&cs, i).or_else(|| fim_da_string(&cs, i));
        if let Some(end) = end {
            match class_of_literal(&cs[i..end], classes) {
                Some(class) => {
                    out.push('"');
                    out.push_str(class);
                    out.push('"');
                    for _ in 0..(end - i).saturating_sub(class.chars().count() + 2) {
                        out.push(' ');
                    }
                }
                None => blank(&cs, i, end, &mut out),
            }
            i = end;
            continue;
        }

        if let Some(end) = fim_do_caractere(&cs, i) {
            blank(&cs, i, end, &mut out);
            i = end;
            continue;
        }

        out.push(c);
        i += 1;
    }
    out
}

/// Copies `from..to` as spaces, keeping the line breaks.
fn blank(cs: &[char], from: usize, to: usize, out: &mut String) {
    for c in &cs[from..to] {
        out.push(if *c == '\n' { '\n' } else { ' ' });
    }
}

/// The class whose name is **the whole literal**, when that is the case.
///
/// Takes the span including the quotes and whatever prefix there is — `"…"`,
/// `r"…"`, `r##"…"##`, `b"…"` — and gives back the name only when the content
/// matches in full. "In full" is the ruler: a name in the middle of a longer
/// text is nobody's decision, it is prose or sample code.
fn class_of_literal<'a>(span: &[char], classes: &[&'a str]) -> Option<&'a str> {
    let text: String = span.iter().collect();
    let content = text
        .trim_start_matches('b')
        .trim_start_matches('r')
        .trim_start_matches('#')
        .strip_prefix('"')?
        .trim_end_matches('#')
        .strip_suffix('"')?
        .to_string();
    classes.iter().find(|c| **c == content).copied()
}

/// The points where the file **decides by class**: the line and the class.
///
/// Searches the text from [`code_with_class_literals`], where only code and the
/// class literals survived — that is why the recognizer may look at a neighbour
/// (`=>`, `==`, `|`) certain that the neighbour is real code, and not the
/// inside of a comment.
pub(super) fn class_dispatches(source: &str, classes: &[&str]) -> Vec<(usize, String)> {
    let code = code_with_class_literals(source, classes);
    let cs: Vec<char> = code.chars().collect();
    let mut found = Vec::new();
    let mut line = 1usize;
    let mut i = 0usize;

    while i < cs.len() {
        if cs[i] == '\n' {
            line += 1;
            i += 1;
            continue;
        }
        if cs[i] != '"' {
            i += 1;
            continue;
        }
        let Some(k) = cs[i + 1..].iter().position(|c| *c == '"') else {
            break;
        };
        let end = i + 1 + k + 1;
        let content: String = cs[i + 1..end - 1].iter().collect();
        if classes.contains(&content.as_str()) && context_decides(&cs, i, end) {
            found.push((line, content));
        }
        i = end;
    }
    found
}

/// Is the literal at `start..end` sitting where it **decides**?
///
/// Four shapes, and they are the four the tree uses:
///
/// ```text
/// match c.class.as_str() { "Button" => …          match arm
/// if n.class == "RadioButton"                     comparison
/// "TreeView" | "ComboBox" => …                    pattern alternative
/// matches!(class, "ActionList" | "Timer")         the one-line match
/// ```
///
/// What it does **not** accuse, deliberately: `f("Button")`, `Node::new(id,
/// "Button")`, `("item_btn", "Button")` — the name passed as an argument. That
/// line is what separates the test fixture, which the tree has by the hundred,
/// from the dispatcher; see the measurement in [`THRESHOLD`].
fn context_decides(cs: &[char], start: usize, end: usize) -> bool {
    // Backwards: `== "Class"`, `!= "Class"` and the tail of an alternative.
    let before: String = cs[..start].iter().rev().take(64).collect();
    let before: String = before.chars().rev().collect();
    let before = before.trim_end();
    if before.ends_with("==") || before.ends_with("!=") || before.ends_with('|') {
        return true;
    }

    // Forwards, and the `|` FIRST, before skipping any closer: `"Class" |`
    // only exists as a pattern (`&str` has no bitwise or), while
    // `f("Class") || g(…)` is an ordinary expression and decides nothing.
    let mut j = end;
    while cs.get(j).is_some_and(|c| c.is_whitespace()) {
        j += 1;
    }
    if cs.get(j) == Some(&'|') {
        return true;
    }
    // A `match` arm, with or without the closers of a pattern wrapping the
    // literal (`Some("Class") =>`).
    loop {
        while cs.get(j).is_some_and(|c| c.is_whitespace()) {
            j += 1;
        }
        match cs.get(j) {
            Some(')') | Some(']') => j += 1,
            _ => break,
        }
    }
    if cs.get(j) == Some(&'=') && cs.get(j + 1) == Some(&'>') {
        return true;
    }

    // `matches!(x, "Class")`: the literal is a pattern argument, and
    // `matches!` is the one-line `match`.
    inside_matches(cs, start)
}

/// Is the open paren wrapping `start` the one of a `matches!`?
///
/// Walks backwards matching parens until it finds the open one, and stops at
/// the first `;`, `{` or `}` — without that brake, a lone literal would make
/// the search sweep the whole file backwards at every point, and one day it
/// would find another function's `matches!`.
fn inside_matches(cs: &[char], start: usize) -> bool {
    let mut depth = 0i32;
    let mut j = start;
    while j > 0 {
        j -= 1;
        match cs[j] {
            ')' => depth += 1,
            '(' => {
                if depth == 0 {
                    let before: String = cs[..j].iter().rev().take(16).collect();
                    let before: String = before.chars().rev().collect();
                    return before.trim_end().ends_with("matches!");
                }
                depth -= 1;
            }
            ';' | '{' | '}' => return false,
            _ => {}
        }
    }
    false
}

#[cfg(test)]
#[path = "test/class_dispatch_gate/test.rs"]
mod tests;
