//! The capability matrix: every action against every kind of project.
//!
//! The IDE hosts many projects, and each kind must see only what applies to
//! it — a console project offers no form commands, a library has nothing to
//! run. The golden file `matrix.golden` is the whole table; a change to any
//! action's gate shows up here as a diff to review, not as a surprise
//! in the menu. Bless a reviewed change with `CRAWRAD_BLESS=1`.

use crate::{Args, Author, Context, DebugSession, Doc, EditorText, Project, catalogue};
use crawrad_designer::clipboard::Clipboard;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

const GOLDEN: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/test/capability_matrix/matrix.golden"
);

/// The kinds of project, as the IDE measures them, plus the debugger stopped.
const PROFILES: [&str; 6] = ["none", "console", "library", "form", "form-sel", "debug"];

fn form() -> Doc {
    let mut d = Document::new("form_main", "Form");
    for id in ["Button1", "Button2"] {
        d.add_child("form_main", Node::new(id, "Button")).unwrap();
    }
    Doc::new(d)
}

fn context<'a>(profile: &str, doc: &'a Doc, r: &'a Registry, a: &'a Clipboard) -> Context<'a> {
    let designer = profile.starts_with("form");
    let project = Project {
        open: profile != "none",
        has_binary: profile != "none" && profile != "library",
        has_forms: designer,
        is_workspace: false,
        debug: match profile {
            "debug" => DebugSession::Stopped,
            _ => DebugSession::None,
        },
    };
    Context {
        doc: designer.then_some(doc),
        registry: r,
        clipboard: a,
        args: Args::none(),
        project,
        author: Author::User,
        editor_dirty: false,
        text: EditorText {
            code_in_front: !designer && profile != "none",
            ..Default::default()
        },
    }
}

fn matrix() -> String {
    let (r, a) = (Registry::builtin(), Clipboard::default());
    let (plain, mut selected) = (form(), form());
    selected.select(vec!["Button1".into(), "Button2".into()]);
    let mut out = format!("{:<40} {}\n", "action", PROFILES.join(" "));
    for action in catalogue() {
        let cells: Vec<String> = PROFILES
            .iter()
            .map(|p| {
                let doc = if *p == "form-sel" { &selected } else { &plain };
                let on = action.enabled.allows(&context(p, doc, &r, &a));
                format!("{:<w$}", if on { "x" } else { "." }, w = p.len())
            })
            .collect();
        out.push_str(&format!("{:<40} {}\n", action.id, cells.join(" ")));
    }
    out
}

#[test]
fn the_matrix_matches_the_golden_table() {
    let now = matrix();
    if std::env::var_os("CRAWRAD_BLESS").is_some() {
        std::fs::write(GOLDEN, &now).unwrap();
        return;
    }
    let golden = std::fs::read_to_string(GOLDEN).unwrap_or_default();
    let golden = golden.replace("\r\n", "\n");
    let changed: Vec<_> = (now.lines().zip(golden.lines()))
        .filter(|(n, g)| n != g)
        .map(|(n, g)| format!("  now:    {n}\n  golden: {g}"))
        .collect();
    assert!(
        changed.is_empty() && now.lines().count() == golden.lines().count(),
        "the capability matrix changed — review, then bless with CRAWRAD_BLESS=1:\n{}",
        changed.join("\n")
    );
}
