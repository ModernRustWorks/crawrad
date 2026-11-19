//! The gate over the editor tab strip.

use super::*;

fn tabs(paths: &[&str]) -> EditorTabs {
    let mut t = EditorTabs::default();
    for p in paths {
        t.open(PathBuf::from(p));
    }
    t
}

fn one_line(t: &EditorTabs) -> Vec<String> {
    t.labels().iter().map(|l| l.one_line()).collect()
}

/// **A fresh strip is the Design tab alone, and it is in front.**
#[test]
fn a_fresh_strip_shows_the_canvas() {
    let t = EditorTabs::default();
    assert!(t.is_empty());
    assert_eq!(t.active_index(), None, "None IS the Design tab");
    assert_eq!(t.active_file(), None);
}

/// **Opening a file puts it in front; reopening it focuses instead of doubling.**
///
/// The mutation this refuses is the `push` with no lookup. It would draw two
/// tabs over one file — two buffers to keep in step, and the second is always
/// the stale one.
#[test]
fn reopening_a_file_focuses_it_instead_of_duplicating() {
    let mut t = tabs(&["a/one.rs", "a/two.rs"]);
    assert_eq!(t.len(), 2);
    assert_eq!(t.active_file(), Some(Path::new("a/two.rs")));

    let i = t.open(PathBuf::from("a/one.rs"));
    assert_eq!(i, 0, "it is the tab that was already there");
    assert_eq!(t.len(), 2, "no second tab over the same file");
    assert_eq!(t.active_file(), Some(Path::new("a/one.rs")));
}

/// **Closing the front tab hands the front to the RIGHT.**
#[test]
fn closing_the_front_tab_walks_forward() {
    let mut t = tabs(&["a/one.rs", "a/two.rs", "a/three.rs"]);
    t.activate(0);
    t.close(0);
    assert_eq!(
        t.active_file(),
        Some(Path::new("a/two.rs")),
        "the neighbour to the right takes the front, so closing several in a \
         row walks forward instead of bouncing to the start"
    );
}

/// **The LAST tab hands the front to the left, and the last of all to the canvas.**
#[test]
fn closing_the_last_tab_falls_back_left_and_then_to_the_canvas() {
    let mut t = tabs(&["a/one.rs", "a/two.rs"]);
    t.activate(1);
    t.close(1);
    assert_eq!(t.active_file(), Some(Path::new("a/one.rs")));

    t.close(0);
    assert_eq!(t.active_index(), None, "nothing left but the canvas");
    assert!(t.is_empty());
}

/// **Closing a tab BEFORE the active one keeps the same file in front.**
///
/// This is the off-by-one the type could not prevent: the indices after the
/// removed one shift down, so an untouched `active` would suddenly point at
/// the neighbour. The test asserts the FILE, not the index, because the index
/// is exactly what is allowed to change.
#[test]
fn closing_a_tab_before_the_active_one_does_not_move_the_front() {
    let mut t = tabs(&["a/one.rs", "a/two.rs", "a/three.rs"]);
    t.activate(2);
    t.close(0);
    assert_eq!(
        t.active_file(),
        Some(Path::new("a/three.rs")),
        "closing a tab to the LEFT must not change which file is being read"
    );
    assert_eq!(t.active_index(), Some(1), "but its index did move");
}

/// **Closing a tab AFTER the active one changes nothing at all.**
#[test]
fn closing_a_tab_after_the_active_one_changes_nothing() {
    let mut t = tabs(&["a/one.rs", "a/two.rs", "a/three.rs"]);
    t.activate(0);
    t.close(2);
    assert_eq!(t.active_index(), Some(0));
    assert_eq!(t.active_file(), Some(Path::new("a/one.rs")));
}

/// **Closing while the canvas is in front leaves the canvas in front.**
#[test]
fn closing_from_the_canvas_keeps_the_canvas() {
    let mut t = tabs(&["a/one.rs", "a/two.rs"]);
    t.show_design();
    t.close(0);
    assert_eq!(t.active_index(), None);
    assert_eq!(t.len(), 1);
}

/// **An index past the end does nothing — it does not panic and does not move.**
#[test]
fn an_index_past_the_end_is_ignored() {
    let mut t = tabs(&["a/one.rs"]);
    t.activate(7);
    assert_eq!(
        t.active_index(),
        Some(0),
        "a bad index must not move the front"
    );
    t.close(7);
    assert_eq!(t.len(), 1, "and must not remove anything");
}

/// **A name nobody shares carries no hint.**
///
/// The quiet case is the common one, and the mutation this refuses is hinting
/// on every tab: it would double the width of a strip that exists to be
/// glanced at.
#[test]
fn a_unique_name_says_only_its_name() {
    let t = tabs(&[
        "crates/crawrad-ide/src/ide_app.rs",
        "crates/crawrad-ir/src/lib.rs",
    ]);
    let labels = t.labels();
    assert!(labels.iter().all(|l| l.hint.is_none()));
    assert_eq!(one_line(&t), vec!["ide_app.rs", "lib.rs"]);
}

/// **Two files with the same name get the folder that tells them apart.**
///
/// This is the owner's screenshot: `inspector.rs  …\ide_app` next to the
/// other `inspector.rs`.
#[test]
fn a_repeated_name_gets_one_folder() {
    let t = tabs(&[
        "crates/crawrad-ide/src/inspector.rs",
        "crates/crawrad-ide/src/ide_app/inspector.rs",
    ]);
    assert_eq!(
        one_line(&t),
        vec!["inspector.rs  …/src", "inspector.rs  …/ide_app"]
    );
}

/// **One folder is not always enough, and then it takes two.**
///
/// Both files sit in a folder called `src`, so `…/src` would name them both.
/// The rule is the SHORTEST run that separates them, not a fixed one.
#[test]
fn it_climbs_until_the_names_really_differ() {
    let t = tabs(&[
        "crates/crawrad-ir/src/lib.rs",
        "crates/crawrad-doc/src/lib.rs",
    ]);
    assert_eq!(
        one_line(&t),
        vec!["lib.rs  …/crawrad-ir/src", "lib.rs  …/crawrad-doc/src"],
        "one folder said `src` for both; two is the shortest that separates"
    );
}

/// **A third file joins the group and only the group pays.**
///
/// The tab with a unique name stays short while the two that collide grow —
/// disambiguation is per group, not per strip.
#[test]
fn only_the_colliding_group_gets_a_hint() {
    let t = tabs(&["a/x/lib.rs", "a/y/lib.rs", "a/z/main.rs"]);
    assert_eq!(one_line(&t), vec!["lib.rs  …/x", "lib.rs  …/y", "main.rs"]);
}

/// **Windows paths are shown with forward slashes.**
///
/// The hint is read, never parsed back into a path. A backslash in the middle
/// of a label reads like an escape.
#[test]
fn the_hint_uses_forward_slashes_on_every_platform() {
    let t = tabs(&[r"c:\proj\alpha\lib.rs", r"c:\proj\beta\lib.rs"]);
    for line in one_line(&t) {
        assert!(
            !line.contains('\\'),
            "a backslash reached the label: {line}"
        );
    }
}

/// **A path with no folder above it still gets a name.**
#[test]
fn a_bare_file_name_survives() {
    let t = tabs(&["main.rs"]);
    assert_eq!(one_line(&t), vec!["main.rs"]);
}

/// **Two identical paths cannot be separated, and the labels say nothing false.**
///
/// `open` never builds this, but `labels_for` is public and pure. The loop
/// that climbs folders has to stop, and the honest stop is no hint at all —
/// a hint that hints at nothing is worse than none.
#[test]
fn identical_paths_get_no_invented_hint() {
    let same = PathBuf::from("a/b/lib.rs");
    let labels = labels_for(&[same.clone(), same]);
    assert_eq!(labels.len(), 2);
    assert!(
        labels
            .iter()
            .all(|l| l.hint.is_none() && l.name == "lib.rs"),
        "no run of folders separates a path from itself: {labels:?}"
    );
}

/// **`close_all` leaves the canvas and nothing else.**
#[test]
fn close_all_leaves_the_canvas() {
    let mut t = tabs(&["a/one.rs", "a/two.rs"]);
    t.close_all();
    assert!(t.is_empty());
    assert_eq!(t.active_index(), None);
}

/// Each tab keeps its caret while it is open; a file that is not open
/// is not remembered, and closing a tab forgets its caret.
#[test]
fn a_tab_keeps_its_caret_until_it_closes() {
    let mut tabs = EditorTabs::default();
    let a = PathBuf::from("a.rs");
    tabs.remember_caret(&a, 3, 1);
    assert_eq!(tabs.caret_of(&a), None);
    tabs.open(a.clone());
    tabs.remember_caret(&a, 3, 1);
    assert_eq!(tabs.caret_of(&a), Some((3, 1)));
    tabs.close(0);
    tabs.open(a.clone());
    assert_eq!(tabs.caret_of(&a), None);
}

/// **`retain` closes what it is told to, and the front follows each close.**
#[test]
fn retain_closes_the_rest_and_keeps_the_front_honest() {
    let mut t = tabs(&["a/one.rs", "a/two.rs", "a/three.rs"]);
    t.activate(2);
    t.retain(|p| p != Path::new("a/two.rs"));
    let left: Vec<_> = t.files().iter().map(|p| p.display().to_string()).collect();
    assert_eq!(left, ["a/one.rs", "a/three.rs"]);
    assert_eq!(t.active_file(), Some(Path::new("a/three.rs")));
    t.retain(|_| false);
    assert!(t.is_empty() && t.active_index().is_none());
}
