//! The gate of the startup warm-up of the Components lists.
//!
//! The warm-up instantiates one row of every `PortalList` template of the
//! Components panel, so their shaders compile at startup instead of on the
//! first click of the rail (measured: 1395-1752 ms before, 11-12 ms after).
//!
//! Nothing complains when that list drifts from the DSL: `PortalList::item`
//! with a template it does not have builds nothing, and a template the list
//! forgot is simply compiled on the first click again — the delay comes back
//! and every test stays green. This gate reads the DSL and compares.

use crate::ide_app::IdeApp;
use makepad_widgets::LiveId;

/// The templates declared inside the `<list> = <PortalList> { ... }` block of
/// `paineis.rs`: every `Name = <...>` line whose name starts upper-case.
fn templates_in_dsl(list: &str) -> Vec<String> {
    let dsl = include_str!("../../ide_app/panels/side_panels.rs");
    let open = format!("{list} = <PortalList> {{");
    let start = dsl
        .find(&open)
        .unwrap_or_else(|| panic!("`{open}` is gone from paineis.rs"));
    let mut depth = 0usize;
    let mut body_end = dsl.len();
    for (i, c) in dsl[start..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    body_end = start + i;
                    break;
                }
            }
            _ => {}
        }
    }
    dsl[start + open.len()..body_end]
        .lines()
        .map(str::trim)
        .filter(|l| !l.starts_with("//"))
        .filter_map(|l| l.split_once(" = <").map(|(name, _)| name.trim()))
        .filter(|name| name.chars().next().is_some_and(char::is_uppercase))
        .map(str::to_string)
        .collect()
}

#[test]
fn warm_up_covers_every_template() {
    for list in ["toolbox_list", "structure_list"] {
        let id = LiveId::from_str(list);
        let (_, warmed) = IdeApp::WARM_UP_LISTS
            .iter()
            .find(|(path, _)| path.last() == Some(&id))
            .unwrap_or_else(|| panic!("`{list}` is not warmed up at startup"));
        let mut declared = templates_in_dsl(list);
        let mut warmed: Vec<String> = warmed.iter().map(|s| s.to_string()).collect();
        declared.sort();
        warmed.sort();
        assert!(
            !declared.is_empty(),
            "no template found in `{list}`: the reader lost its view"
        );
        assert_eq!(
            warmed, declared,
            "the warm-up of `{list}` and its DSL disagree — a template missing \
             from `IdeApp::WARM_UP_LISTS` compiles its shader on the first click again"
        );
    }
}

#[test]
fn warm_up_lists_nothing_else() {
    // Two lists live in the Components panel; a third entry would be a list
    // the gate above does not read.
    assert_eq!(IdeApp::WARM_UP_LISTS.len(), 2);
}
