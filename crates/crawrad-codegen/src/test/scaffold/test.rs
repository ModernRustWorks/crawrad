//! Every project template's scaffold is code that compiles: its braces close.

use crate::AlvoGerador;
use crate::scaffold::{scaffold_app_rs, scaffold_main_rs};

const TEMPLATES: &[&str] = &[
    "gui_app",
    "console_app",
    "library_crate",
    "package_plugin",
    "wasm_app",
    "mobile_app",
    "multiwindow_app",
];

/// The depth never goes below zero and ends at zero.
fn balanced(code: &str) -> bool {
    let mut depth = 0i64;
    for c in code.chars() {
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return false;
        }
    }
    depth == 0
}

/// The Mobile template's `live_design!` lacked one `}` and the new project
/// did not compile (08/11).
#[test]
fn every_template_scaffold_closes_its_braces() {
    for alvo in AlvoGerador::TODOS {
        for t in TEMPLATES {
            let main = scaffold_main_rs(*alvo, "demo", t);
            assert!(balanced(&main), "{t} / {alvo:?}: main.rs\n{main}");
            if let Some(app) = scaffold_app_rs(*alvo, "demo", t) {
                assert!(balanced(&app), "{t} / {alvo:?}: app.rs\n{app}");
            }
        }
    }
}

#[test]
fn no_makepad_template_app_rs_contains_vec4_in_live_design() {
    for t in TEMPLATES {
        let app = scaffold_app_rs(AlvoGerador::Makepad, "demo", t);
        if let Some(live_design) = app
            .as_deref()
            .and_then(|a| a.find("live_design! {").map(|s| &a[s..]))
        {
            assert!(
                !live_design.contains("vec4("),
                "{t}: app.rs contains vec4( in live_design!:\n{live_design}"
            );
        }
    }
}

/// `#26262e` in the DSL is lexed by Rust as a number with an empty
/// exponent — the multi-window template did not compile. A colour that starts
/// with a digit is written `#x…`.
#[test]
fn no_makepad_template_colour_starts_with_a_digit() {
    for t in TEMPLATES {
        let app = scaffold_app_rs(AlvoGerador::Makepad, "demo", t).unwrap_or_default();
        let bad: Vec<&str> = app
            .match_indices('#')
            .filter(|(i, _)| app[i + 1..].starts_with(|c: char| c.is_ascii_digit()))
            .map(|(i, _)| &app[i..(i + 8).min(app.len())])
            .collect();
        assert!(bad.is_empty(), "{t}: {bad:?}");
    }
}
