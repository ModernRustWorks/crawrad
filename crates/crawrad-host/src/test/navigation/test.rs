//! The jump history and which files are not the user's.

use super::{
    Choices, DEPTH, History, Pending, Place, is_foreign, locked, place_label, tab_icon, word_at,
};
use std::path::PathBuf;

fn at(file: &str, line: usize) -> Place {
    Place {
        file: PathBuf::from(file),
        line,
        byte: 0,
    }
}

/// **Back returns to the jump's origin; Forward returns to where Back left.**
#[test]
fn back_and_forward_walk_the_jumps() {
    let mut h = History::default();
    h.jumped_from(at("main.rs", 10)); // main.rs:10 -> lib.rs:3
    h.jumped_from(at("lib.rs", 3)); // lib.rs:3 -> std/string.rs:400
    assert_eq!(h.back(at("string.rs", 400)), Some(at("lib.rs", 3)));
    assert_eq!(h.back(at("lib.rs", 3)), Some(at("main.rs", 10)));
    assert_eq!(h.back(at("main.rs", 10)), None, "nothing older");
    assert_eq!(h.forward(at("main.rs", 10)), Some(at("lib.rs", 3)));
    assert_eq!(h.forward(at("lib.rs", 3)), Some(at("string.rs", 400)));
    assert_eq!(h.forward(at("string.rs", 400)), None);
}

/// **A new jump after going back forgets the forward side.**
#[test]
fn a_new_jump_drops_forward() {
    let mut h = History::default();
    h.jumped_from(at("a.rs", 1));
    let _ = h.back(at("b.rs", 2));
    assert!(h.can_go_forward());
    h.jumped_from(at("a.rs", 1));
    assert!(!h.can_go_forward());
}

/// **The same origin twice in a row is one entry; the oldest fall off.**
#[test]
fn repeats_collapse_and_the_depth_is_bounded() {
    let mut h = History::default();
    h.jumped_from(at("a.rs", 1));
    h.jumped_from(at("a.rs", 1));
    assert_eq!(h.back(at("b.rs", 2)), Some(at("a.rs", 1)));
    assert!(!h.can_go_back());

    let mut h = History::default();
    for i in 0..DEPTH + 5 {
        h.jumped_from(at("a.rs", i));
    }
    let mut n = 0;
    let mut here = at("z.rs", 0);
    while let Some(p) = h.back(here.clone()) {
        here = p;
        n += 1;
    }
    assert_eq!(n, DEPTH);
    assert_eq!(here, at("a.rs", 5), "the five oldest fell off");
}

/// **std and downloaded crates are foreign; the project is not**, whatever
/// the slashes and the case.
#[test]
fn toolchain_and_registry_are_foreign() {
    let roots = vec![
        PathBuf::from(r"C:\Users\me\.rustup"),
        PathBuf::from(r"C:\Users\me\.cargo\registry"),
    ];
    let std_file = PathBuf::from(
        "c:/users/me/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/src/rust/library/alloc/src/string.rs",
    );
    assert!(is_foreign(&std_file, &roots));
    assert!(is_foreign(
        &PathBuf::from(r"C:\Users\me\.cargo\registry\src\index\serde-1.0\src\lib.rs"),
        &roots
    ));
    assert!(!is_foreign(
        &PathBuf::from(r"C:\Users\me\proj\src\main.rs"),
        &roots
    ));
    assert!(
        !is_foreign(&PathBuf::from(r"C:\Users\me\.rustup-notes\a.rs"), &roots),
        "a sibling with the same prefix is not inside"
    );
    assert!(!is_foreign(
        &PathBuf::from(r"C:\Users\me\.cargo\bin\x.rs"),
        &roots
    ));
}

/// **Only the newest ticket's answer counts, and only once.**
#[test]
fn only_the_newest_ticket_answers() {
    let mut h = History::default();
    let old = h.ask(at("a.rs", 1));
    let new = h.ask(at("a.rs", 9));
    assert_eq!(h.answered(old), None, "stale");
    assert_eq!(
        h.pending_ticket(),
        Some(new),
        "still waiting for the newest"
    );
    assert_eq!(h.answered(new), Some(at("a.rs", 9)));
    assert_eq!(h.answered(new), None, "once");
    assert_eq!(h.pending_ticket(), None);
}

/// **The name under the caret**, from its start, its middle or its end.
#[test]
fn the_word_under_the_caret() {
    let line = "    let é_total = soma(x);";
    let at_ = |w: &str, k: usize| line.find(w).unwrap() + k;
    assert_eq!(word_at(line, at_("é_total", 0)), "é_total");
    assert_eq!(word_at(line, at_("é_total", 4)), "é_total");
    assert_eq!(
        word_at(line, at_("soma", 4)),
        "soma",
        "right after the name"
    );
    assert_eq!(
        word_at(line, at_("é_total", 1)),
        "é_total",
        "inside a character"
    );
    assert_eq!(word_at(line, 0), "");
}

/// **The form's generated source is locked like std; the user's unit is not.**
#[test]
fn the_generated_form_source_is_locked() {
    let roots = vec![PathBuf::from(r"C:\Users\me\.rustup")];
    let sfx = ".designer.rs";
    let p = |s: &str| PathBuf::from(s);
    assert!(locked(
        &p(r"C:\proj\src\form_main.designer.rs"),
        &roots,
        sfx
    ));
    assert!(!locked(&p(r"C:\proj\src\form_main.rs"), &roots, sfx));
    assert!(
        !locked(&p(r"C:\proj\src\designer.rs"), &roots, sfx),
        "a file named just designer.rs is the user's"
    );
    assert!(locked(&p(r"C:\Users\me\.rustup\x\string.rs"), &roots, sfx));
}

/// **The tab says why it refuses typing:** a padlock, else crab or page.
#[test]
fn the_tab_icon_shows_the_lock() {
    let roots = vec![PathBuf::from("C:/home/.rustup")];
    let icon = |f: &str| tab_icon(&PathBuf::from(f), &roots, ".designer.rs");
    assert_eq!(icon("C:/p/src/form_main.designer.rs"), "\u{1F512} ");
    assert_eq!(icon("C:/home/.rustup/x/string.rs"), "\u{1F512} ");
    assert_eq!(icon("C:/p/src/main.rs"), "\u{1F980} ");
    assert_eq!(icon("C:/p/Cargo.toml"), "\u{1F4C4} ");
}

/// **Several definitions: choosing one gives it and the origin, once.**
#[test]
fn choosing_among_definitions() {
    let mut c = Choices::default();
    assert!(!c.is_open());
    c.offer(vec![at("main.rs", 3), at("main.rs", 0)], at("main.rs", 4));
    assert!(c.is_open());
    assert_eq!(
        c.to_json(),
        serde_json::json!({ "open": true, "items": ["main.rs:4", "main.rs:1"] })
    );
    assert_eq!(c.choose(5), None, "out of range keeps the list");
    assert!(c.is_open());
    assert_eq!(c.choose(1), Some((at("main.rs", 0), at("main.rs", 4))));
    assert!(!c.is_open());
    assert_eq!(
        c,
        Choices::default(),
        "nothing stale left, not even the origin"
    );
    assert_eq!(c.choose(0), None, "once");
    c.offer(vec![at("a.rs", 1)], at("b.rs", 2));
    c.close();
    assert_eq!(c, Choices::default());
}

/// **The list says file, 1-based line and folder.**
#[test]
fn a_place_as_the_list_shows_it() {
    let p = Place {
        file: PathBuf::from("C:/proj/src/main.rs"),
        line: 3,
        byte: 8,
    };
    assert_eq!(place_label(&p), "main.rs:4  src");
}

/// **A format answer counts only for the newest request, once.**
#[test]
fn a_format_answer_counts_for_the_newest_request() {
    let mut p = Pending::default();
    let old = p.ask(PathBuf::from("a.rs"), 1);
    let new = p.ask(PathBuf::from("a.rs"), 2);
    assert_eq!(p.answered(old), None);
    assert_eq!(p.ticket(), Some(new));
    assert_eq!(p.answered(new), Some((PathBuf::from("a.rs"), 2)));
    assert_eq!(p.answered(new), None);
}
