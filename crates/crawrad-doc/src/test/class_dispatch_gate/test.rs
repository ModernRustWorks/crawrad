//! The three tests of the class dispatch gate: **the sweep**, **the gate's
//! gate** and **the per-component file**.
//!
//! The recognizer lives in the module; what charges it lives here. That split
//! is this house's rule for test files (`src/test/<module>/test.rs`), and it
//! fits this gate for the same reason it fits its sibling: this file writes ten
//! class names in full, inside make-believe `match`es, and **it is swept too**
//! by the first test. If the lexer breaks, this file is the first to accuse.

use super::{CATALOG_CRATE, EXCEPTIONS, THRESHOLD, catalog_classes, class_dispatches};
use crate::silenced_lint_gate::{caminho_curto, raiz_dos_crates, varrer};
use std::collections::BTreeSet;

/// How many `.rs` files the sweep must find, at a minimum.
///
/// Measured on 15/09/2026: **224** under `crates/`, not counting `target/`. The
/// floor is generous because a new file is ordinary life, and what it chases is
/// the **blind** sweep: a root that moved, a `read_dir` that did not open, a
/// gate that starts finding nothing and goes green for it.
const MEASURED_FILES: usize = 150;

/// How many classes the catalog must hand back, at a minimum.
///
/// Twenty on 15/09/2026. With no classes there is nothing to look for, and a
/// gate without a needle stays green forever — the same care
/// `portao_do_comando_por_alvo` takes with an empty `$ALVOS`.
const MEASURED_CLASSES: usize = 15;

/// How many decision points the sweep must **see** across the whole tree.
///
/// Measured on 15/09/2026: **130** outside `crawrad-registry`, including those in
/// the two files with a declared exception — 48 in `crawrad-runtime-egui` alone.
/// This floor is the opposite of the other one: it guarantees the gate keeps
/// **seeing** the dispatch that exists. If it drops without the debt having
/// been paid, the recognizer went blind — and blindness here looks exactly like
/// success.
const MEASURED_POINTS: usize = 100;

/// How many `componentes/`/`components/` files the tree has, at a minimum.
///
/// Measured on 15/09/2026: **77**, not counting the `mod.rs` files — 19 in
/// `crawrad-registry`, 24 in `crawrad-runtime-makepad`, 16 in `codegen/targets/egui`
/// and 18 in `codegen/targets/makepad`. They are living proof that the path
/// exists and is already walked; the test that counts them would be empty
/// without this floor.
const MEASURED_COMPONENT_FILES: usize = 40;

/// How many classes a per-component file may decide, at most.
///
/// **Two.** The largest of them today decides about **one** — its own — and the
/// slack of one exists because `page_control.rs` legitimately answers for
/// `PageControl` and `TabSheet`, which are a single concept. The ceiling is
/// tight on purpose: it is what keeps the absence of a path exclusion (see §2
/// of the module) safe, instead of letting it become the next dispatcher's
/// hiding place.
const COMPONENT_FILE_CEILING: usize = 2;

/// A file is a component file when it lives in a components folder.
fn is_component_file(name: &str) -> bool {
    (name.contains("/componentes/") || name.contains("/components/")) && !name.ends_with("/mod.rs")
}

#[test]
fn no_file_dispatches_over_the_whole_catalog() {
    let root = raiz_dos_crates();
    let classes = catalog_classes();
    assert!(
        classes.len() >= MEASURED_CLASSES,
        "the catalog handed back only {} class(es) — with no needle this gate \
         has nothing to look for and stays green forever",
        classes.len()
    );

    let mut files = Vec::new();
    varrer(&root, &mut files);
    files.sort();
    assert!(
        files.len() >= MEASURED_FILES,
        "the sweep found {} file(s) — the crates path moved, and a gate that \
         cannot find what to charge stays green forever",
        files.len()
    );

    let mut complaints: Vec<String> = Vec::new();
    let mut points = 0usize;
    let mut covered: BTreeSet<&str> = BTreeSet::new();

    for file in &files {
        let Ok(source) = std::fs::read_to_string(file) else {
            continue;
        };
        let name = caminho_curto(file, &root);
        if name.contains(CATALOG_CRATE) {
            continue;
        }
        let found = class_dispatches(&source, &classes);
        points += found.len();

        let distinct: BTreeSet<&str> = found.iter().map(|(_, c)| c.as_str()).collect();
        if distinct.len() <= THRESHOLD {
            continue;
        }
        if let Some((target, _)) = EXCEPTIONS.iter().find(|(target, _)| name.ends_with(target)) {
            covered.insert(target);
            continue;
        }
        let (line, _) = found[0];
        complaints.push(format!(
            "{name}:{line}: decides the behaviour of {} catalog classes ({} \
             points) — {distinct:?}",
            distinct.len(),
            found.len()
        ));
    }

    assert!(
        points >= MEASURED_POINTS,
        "the sweep saw only {points} dispatch point(s) across the whole tree, \
         and expected at least {MEASURED_POINTS} — either the debt was paid \
         (and then this floor goes down, along with THRESHOLD), or the \
         recognizer went blind; blindness here looks exactly like success"
    );

    let dead: Vec<&str> = EXCEPTIONS
        .iter()
        .map(|(target, _)| *target)
        .filter(|target| !covered.contains(target))
        .collect();
    assert!(
        dead.is_empty(),
        "these exceptions cover no dispatcher at all: {dead:?} — either the \
         file is gone, or the debt was PAID and the licence stayed open for the \
         next one to be born in the same place. Delete them."
    );

    assert!(
        complaints.is_empty(),
        "CENTRAL DISPATCH BY CLASS — a file is answering for the whole \
         catalog:\n{}\n\n\
         The component is the one responsible for its own feature. The way out \
         is one file per component, with the component resolving and handing \
         the result back: that is how `crawrad-runtime-makepad` does it — 19 \
         per-class files under `src/componentes/`, a `mod.rs` that is a catalog \
         without behaviour, and ZERO class dispatch points in the whole crate. \
         `crawrad-codegen/src/targets/makepad/components/` does the same on the \
         generator side.\n\n\
         The owner's test: *does adding a new component cost CREATING ONE \
         FILE, or editing one central place?* If the second answer is the true \
         one, there is a double maintenance point — and it does not warn you \
         when it diverges: in 09/2026 a 14-class list in the shell and its twin \
         in the DSL diverged on three (`ActionList`, `ImageList`, `Timer`), \
         which became three dead rows on screen without a single red.\n\n\
         If this file is debt that cannot be paid right now, the way out is \
         written: one line in `EXCEPTIONS`, with the reason and the issue, \
         visible in the diff.",
        complaints.join("\n")
    );
}

/// **The gate's gate.** The sweep above is only worth something if the
/// recognizer knows the difference between deciding by class, quoting a class
/// and using one as data.
///
/// Each case is a whole source, not a slice of a line, because that is how the
/// recognizer works: it needs the context to decide. The cases that **pass**
/// are the ones that matter — they are the ones a gate made of `grep` would
/// reject, and the third of them is the fixture the tree has by the hundred.
///
/// Mutation proof, measured on 15/09/2026:
/// - returning the raw source from `code_with_class_literals` (blanking
///   nothing) drops the three quotation-in-comment and in-string cases;
/// - blanking EVERY string, like the sibling gate's `mascarar`, drops the first
///   four cases at once, and the whole sweep with them;
/// - removing `inside_matches` drops the single-argument `matches!` case.
#[test]
fn the_recognizer_separates_decision_from_data_and_quotation() {
    let classes = catalog_classes();
    let fence = "#".repeat(2);
    let cases: &[(&str, String, bool)] = &[
        (
            "the match arm",
            "match c.class.as_str() {\n    \"Button\" => paint(),\n    _ => {}\n}\n".to_string(),
            true,
        ),
        (
            "the comparison",
            "if n.class == \"RadioButton\" { group(); }\n".to_string(),
            true,
        ),
        (
            "the pattern alternative",
            "match k {\n    \"TreeView\" | \"ComboBox\" => open(),\n    _ => {}\n}\n".to_string(),
            true,
        ),
        (
            "the one-line match",
            "let v = !matches!(class, \"ActionList\" | \"Timer\");\n".to_string(),
            true,
        ),
        (
            "the one-line match, with a single argument",
            "let v = matches!(c.class.as_str(), \"Memo\");\n".to_string(),
            true,
        ),
        (
            "the wrapped pattern",
            "match c.class() {\n    Some(\"GroupBox\") => frame(),\n    _ => {}\n}\n".to_string(),
            true,
        ),
        // --- from here down, everything passes ---
        (
            "the fixture, which is the name as an argument",
            "let n = Node::new(\"btn1\", \"Button\");\nlet e = Node::new(\"e1\", \"Edit\");\n"
                .to_string(),
            false,
        ),
        (
            "the dispatcher quoted inside a string, which is what the generator does",
            "let s = \"match class { \\\"Button\\\" => 1, \\\"Edit\\\" => 2 }\";\n".to_string(),
            false,
        ),
        (
            "the dispatcher inside a raw string, at column 1",
            format!(
                "const A: &str = r{fence}\"\nmatch c {{\n    \"Button\" => 1,\n    \"Memo\" => 2,\n}}\n\"{fence};\n"
            ),
            false,
        ),
        (
            "the quotation in a line comment",
            "// never write `match c { \"Button\" => … }` here\nfn f() {}\n".to_string(),
            false,
        ),
        (
            "the quotation in a doc comment",
            "/// A central `if class == \"Panel\"` is forbidden.\nfn f() {}\n".to_string(),
            false,
        ),
        (
            "the quotation in a block comment, nested",
            "/* /* match c { \"Label\" => 1 } */ still inside */\nfn f() {}\n".to_string(),
            false,
        ),
        (
            "the logical or of two calls, which is no pattern alternative",
            "if used.contains(\"ListBox\") || used.contains(\"ComboBox\") { x(); }\n".to_string(),
            false,
        ),
        (
            "the value on the right of the arrow, which is a map and not a decision",
            "match i {\n    Icon::Push => \"Button\",\n    _ => \"\",\n}\n".to_string(),
            false,
        ),
        (
            "the class in the middle of a longer text",
            "let msg = \"the Button does not fit in the Panel\";\n".to_string(),
            false,
        ),
        (
            "what is not a catalog class",
            "match s {\n    \"ssHorizontal\" | \"ssBoth\" => 1,\n    _ => 0,\n}\n".to_string(),
            false,
        ),
    ];

    let wrong: Vec<&str> = cases
        .iter()
        .filter(|(_, source, accuses)| class_dispatches(source, &classes).is_empty() == *accuses)
        .map(|(name, _, _)| *name)
        .collect();
    assert!(
        wrong.is_empty(),
        "the recognizer answered the opposite of what these cases deserve: {wrong:?}"
    );
}

/// The per-component file decides about **its own** class, and that is right.
///
/// This test is the ruler's other half, and it exists because the module did
/// **not** open a path exclusion for `componentes/`: instead of trusting the
/// folder to be sacred, it measures. As long as the largest component file
/// decides about one class — its own — the threshold of eight absolves them on
/// its own, and nobody needs a licence by path.
///
/// If it goes red, the news is specific: someone put a dispatcher **inside**
/// the components folder, which is the last place a reviewer would look.
#[test]
fn the_per_component_file_answers_only_for_its_own_class() {
    let root = raiz_dos_crates();
    let classes = catalog_classes();
    let mut files = Vec::new();
    varrer(&root, &mut files);

    let mut seen = 0usize;
    let mut complaints: Vec<String> = Vec::new();
    for file in &files {
        let name = caminho_curto(file, &root);
        if !is_component_file(&name) {
            continue;
        }
        seen += 1;
        let Ok(source) = std::fs::read_to_string(file) else {
            continue;
        };
        let distinct: BTreeSet<String> = class_dispatches(&source, &classes)
            .into_iter()
            .map(|(_, c)| c)
            .collect();
        if distinct.len() > COMPONENT_FILE_CEILING {
            complaints.push(format!("{name}: {distinct:?}"));
        }
    }

    assert!(
        seen >= MEASURED_COMPONENT_FILES,
        "the sweep found only {seen} component file(s), and expected at least \
         {MEASURED_COMPONENT_FILES} — either the folders were renamed, or this \
         test started measuring the void"
    );
    assert!(
        complaints.is_empty(),
        "a component file is deciding for the OTHERS:\n{}\n\n\
         The `componentes/` folder is where each class answers for itself; a \
         dispatcher in here is the same defect hidden in the last place anyone \
         would look. What each file may decide is its own class — and the \
         family that is a single concept, like `PageControl` and its \
         `TabSheet`.",
        complaints.join("\n")
    );
}

/// The accused line is the file's line — otherwise the complaint sends the dev
/// to the wrong place, and a lying complaint costs more than no complaint.
#[test]
fn the_complaint_points_at_the_right_line() {
    let classes = catalog_classes();
    let fence = "#".repeat(2);
    let source = format!(
        "//! doc quoting `\"Button\"`\n\
         /* a\n   three-line\n   comment */\n\
         const A: &str = r{fence}\"\nmatch c {{ \"Edit\" => 1 }}\n\"{fence};\n\
         match c.class.as_str() {{\n    \"Memo\" => 1,\n    _ => 0,\n}}\n"
    );
    assert_eq!(
        class_dispatches(&source, &classes),
        vec![(9, "Memo".to_string())]
    );
}
