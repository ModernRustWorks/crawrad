//! Link item…: the newest question's answer, and the link at the caret.

use super::{LinkPick, Linkable};
use std::path::Path;

fn item(name: &str, container: Option<&str>) -> Linkable {
    Linkable {
        name: name.into(),
        container: container.map(Into::into),
        file: "C:/demo/src/lib.rs".into(),
        line: 3,
    }
}

/// **A method links through its type; a module item by its name.**
#[test]
fn the_link_names_the_type_it_belongs_to() {
    assert_eq!(
        item("fetch", Some("RegistryClient")).link(),
        "[`RegistryClient::fetch`]"
    );
    assert_eq!(item("fetch_all", None).link(), "[`fetch_all`]");
    assert_eq!(
        item("fetch", Some("RegistryClient")).label(),
        "RegistryClient::fetch  lib.rs:4"
    );
}

/// **Only the newest question's answer fills the list** — the user typed
/// on, an older answer is stale.
#[test]
fn an_answer_to_an_older_question_is_dropped() {
    let mut pick = LinkPick::default();
    assert_eq!(pick.ask("fe"), None, "closed: nothing is asked");
    pick.open(12, Path::new("C:/demo"));
    let (old, q) = pick.ask(" fe ").expect("asked");
    assert_eq!(q, "fe");
    let (new, _) = pick.ask("fetch").expect("asked again");
    assert!(!pick.answered(old, vec![item("stale", None)]));
    assert!(pick.found.is_empty());
    assert!(pick.answered(new, vec![item("fetch", Some("RegistryClient"))]));
    assert_eq!(pick.labels(), ["RegistryClient::fetch  lib.rs:4"]);
    assert_eq!(pick.pending(), None);
}

/// **A blank name empties the list and asks nothing.**
#[test]
fn a_blank_name_asks_nothing() {
    let mut pick = LinkPick::default();
    pick.open(0, Path::new("C:/demo"));
    let (t, _) = pick.ask("x").expect("asked");
    assert!(pick.answered(t, vec![item("x", None)]));
    assert_eq!(pick.ask("   "), None);
    assert!(pick.found.is_empty());
}

/// **Choosing gives the caret and the link once, and closes.**
#[test]
fn the_choice_is_the_link_at_the_caret_once() {
    let mut pick = LinkPick::default();
    pick.open(42, Path::new("C:/demo"));
    let (t, _) = pick.ask("Not").expect("asked");
    pick.answered(t, vec![item("NotFound", Some("RegistryError"))]);
    assert_eq!(pick.choose(1), None, "no such row");
    assert_eq!(
        pick.choose(0),
        Some((42, "[`RegistryError::NotFound`]".into()))
    );
    assert!(!pick.is_open());
    assert_eq!(pick.choose(0), None);
}

/// **Only the project's own items are listed** — a path dependency's
/// (`C:/demo-dep`, a sibling whose name starts the same) is not, and the
/// server's `c:\\` Windows path still counts as the project's.
#[test]
fn the_items_outside_the_project_are_left_out() {
    let mut pick = LinkPick::default();
    pick.open(0, Path::new("C:/demo/"));
    let (t, _) = pick.ask("fetch").expect("asked");
    let mut dep = item("rule2", None);
    dep.file = "C:/demo-dep/src/test.rs".into();
    let mut ours = item("fetch", Some("RegistryClient"));
    ours.file = r"c:\demo\src\lib.rs".into();
    assert!(pick.answered(t, vec![dep, ours]));
    let names: Vec<&str> = pick.found.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, ["fetch"]);
}
