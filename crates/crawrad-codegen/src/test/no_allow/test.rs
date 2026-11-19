//! **Nothing CrawRAD writes silences a lint.**
//!
//! Not the designer, not the user's file, not `main.rs`, and not in either
//! target. This is the house rule stated by the owner, and it is stated here
//! once so that it does not have to be re-stated per lint:
//!
//! > *"Rule: NO `allow()`. Rust has plenty of tools to suggest the
//! > right code, instead of working around the compiler."*
//!
//! # Why a sweep, and not one assertion per lint
//!
//! The egui designer used to carry `#![allow(non_snake_case, dead_code,
//! clippy::needless_update)]` — three lints, of which **two never fired at
//! all** and the third was hiding real defects: a `CheckBox` whose caption was
//! a literal in the drawing instead of the state field next to it, a `Panel`
//! whose `enabled` no line of the generated code read, a `TabSheet` no field
//! of the form was ever looked at. Each of those would have needed its own
//! assertion, written by someone who already knew it existed.
//!
//! An attribute is one shape and it is cheap to look for, so the sweep is the
//! cheap guard that catches the lint nobody has met yet — including in a target
//! that does not exist today. The makepad `allow(non_snake_case)` assertion is
//! a special case of this file, and is left where it is: a second reader of
//! the same rule costs nothing.
//!
//! # Why the inner attribute is the worst of them
//!
//! An `#![allow(...)]` at the top of the designer silences the **whole file**,
//! and the designer is rewritten whole on every generation — so the warning it
//! hides belongs to someone who cannot fix it and cannot even keep an edit that
//! would. That is the opposite of *a RAD application the compiler checks*, which
//! is the whole claim.

use crate::Gerador;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

/// The attribute shapes that silence a lint. `expect` is `allow` with an
/// expiry date on it, and it silences just the same — a narrower spelling of
/// the same dodge is still the dodge.
const SILENCERS: &[&str] = &["allow(", "expect("];

/// A form with **one of every class in the catalogue**, every event it
/// declares wired to a handler.
///
/// The sweep is what makes this file cover the class that has not been written
/// yet: whoever adds a component to the catalogue walks through here without
/// asking. Events are wired because the payload types the designer emits
/// depend on them — the widest document is the one that emits the most.
fn every_class() -> Document {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the brand new root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
        root.set_name("form_main");
        root.set_text("Form1");
    }
    let mut y = 8.0;
    let mut form_events: Vec<(String, String)> = Vec::new();
    for desc in r.all() {
        if desc.class == "Form" {
            for e in &desc.events {
                form_events.push((
                    e.name.to_string(),
                    crate::handler_padrao("form_main", e.name),
                ));
            }
            continue;
        }
        let id = r.suggest_id(desc.class, |i| d.node(i).is_some());
        let (w, h) = desc.default_size;
        let mut n = r
            .instantiate(desc.class, id.clone(), Rect::new(8.0, y, w, h))
            .expect("the catalogue instantiates what it lists");
        for e in &desc.events {
            n.events
                .insert(e.name.into(), crate::handler_padrao(&id, e.name));
        }
        d.add_child("form_main", n).expect("it goes on the form");
        y += h + 4.0;
    }
    let root = d.node_mut("form_main").expect("the root");
    for (event, handler) in form_events {
        root.events.insert(event, handler);
    }
    d.validate().expect("valid");
    d
}

/// Every generator the crate ships, by name.
fn targets() -> Vec<(&'static str, Box<dyn Gerador>)> {
    vec![
        ("egui", Box::new(crate::GeradorRxp) as Box<dyn Gerador>),
        ("makepad", Box::new(crate::GeradorMakepad)),
    ]
}

/// **The gate.** Not one line of Rust that CrawRAD writes may carry an
/// `allow` or an `expect`.
#[test]
fn nothing_generated_silences_a_lint() {
    let doc = every_class();
    let mut found: Vec<String> = Vec::new();
    let mut swept = 0usize;
    for (target, g) in targets() {
        for file in g
            .gerar(&doc, "app", &crawrad_registry::Registry::builtin())
            .expect("generating")
        {
            if file.caminho.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            swept += 1;
            for (i, line) in file.conteudo.lines().enumerate() {
                for s in SILENCERS {
                    if line.contains(s) {
                        found.push(format!(
                            "{target} · {}:{} · {}",
                            file.caminho.display(),
                            i + 1,
                            line.trim()
                        ));
                    }
                }
            }
        }
    }
    assert!(
        swept >= 6,
        "the sweep checked {swept} `.rs` files — two targets give at least \
         three each, so it stopped measuring"
    );
    assert!(
        found.is_empty(),
        "the generated project silences a lint, and the user cannot fix it — the \
         designer is rewritten on the next generation:\n{}",
        found.join("\n")
    );
}

/// **The mutation this file must catch**, written as a test so it does not
/// need to be done by hand: the sweep looks at the entire emitted text,
/// not a list of files chosen here.
///
/// Without this, an `allow` appearing in a new file — a `lib.rs`, a
/// second form — would slip through unnoticed, and the test above would stay green
/// measuring yesterday's files.
#[test]
fn the_sweep_reads_every_generated_rust_file() {
    let doc = every_class();
    for (target, g) in targets() {
        let files = g
            .gerar(&doc, "app", &crawrad_registry::Registry::builtin())
            .expect("generating");
        let rust: Vec<String> = files
            .iter()
            .filter(|f| f.caminho.extension().is_some_and(|e| e == "rs"))
            .map(|f| f.caminho.display().to_string())
            .collect();
        assert!(
            rust.iter().any(|p| p.contains("designer")),
            "target `{target}` emitted no designer: {rust:?}"
        );
        assert!(
            rust.iter().any(|p| p.ends_with("main.rs")),
            "target `{target}` emitted no `main.rs`: {rust:?}"
        );
        assert!(
            rust.iter()
                .any(|p| p.contains("form_main.rs") && !p.contains("designer")),
            "target `{target}` emitted no user file: {rust:?}"
        );
    }
}
