//! The gate over the strip of open forms.
//!
//! The host held ONE document, so opening a second form replaced the first and the
//! person lost their place. These are the cases that break a tab strip: the front
//! index must follow its own form when a tab behind it closes, and one file must
//! never open in two tabs.

use super::*;

fn a_form(name: &str) -> Doc {
    Doc::new(crawrad_ir::Document::new(name, "Form"))
}

/// Opening a second form no longer throws the first away.
///
/// The host held ONE document, so opening another replaced it and the person
/// lost their place. `doc()` still answers for the form in front, which is how
/// the hundred callers that ask for "the document" kept working unchanged.
#[test]
fn a_second_form_joins_the_strip_instead_of_replacing_the_first() {
    let mut host = HostModel::new();
    host.open_form(a_form("form_main"), Some("a/form_main.crawform".into()));
    host.open_form(a_form("form_1"), Some("a/form_1.crawform".into()));

    assert_eq!(host.forms().len(), 2, "the first form was thrown away");
    assert_eq!(
        host.front(),
        Some(1),
        "the form just opened is not in front"
    );
    assert_eq!(
        host.doc().map(|d| d.ir().root.as_str()),
        Some("form_1"),
        "`doc()` must answer for the form in front"
    );
}

/// One file, one tab — however many times it is opened.
///
/// Two tabs of one file would each hold their own edits, and saving would
/// decide between them silently.
#[test]
fn opening_a_file_that_is_already_open_brings_it_forward() {
    let mut host = HostModel::new();
    let file: PathBuf = "a/form_main.crawform".into();
    let first = host.open_form(a_form("form_main"), Some(file.clone()));
    host.open_form(a_form("form_1"), Some("a/form_1.crawform".into()));
    let again = host.open_form(a_form("form_main"), Some(file));

    assert_eq!(first, again, "the same file opened in a second tab");
    assert_eq!(host.forms().len(), 2, "a duplicate tab was added");
    assert_eq!(host.front(), Some(first), "it did not come forward");
}

/// **Closing a tab behind the front one must not change what is on screen.**
///
/// This is the bug every tab strip is born with: the front index stays put
/// while the forms shift left under it, and the person is suddenly looking at
/// a different form than the one they were editing.
#[test]
fn closing_a_tab_behind_the_front_leaves_the_same_form_in_front() {
    let mut host = HostModel::new();
    host.open_form(a_form("first"), Some("a/first.crawform".into()));
    host.open_form(a_form("second"), Some("a/second.crawform".into()));
    host.open_form(a_form("third"), Some("a/third.crawform".into()));
    host.bring_to_front(2);

    host.close_form(0);

    assert_eq!(host.forms().len(), 2);
    assert_eq!(
        host.doc().map(|d| d.ir().root.as_str()),
        Some("third"),
        "closing a tab behind the front changed the form on screen"
    );
}

/// Closing the front tab brings its right-hand neighbour forward, as every
/// editor does — and the left-hand one at the end of the strip.
#[test]
fn closing_the_front_tab_moves_to_the_neighbour() {
    let mut host = HostModel::new();
    host.open_form(a_form("first"), Some("a/first.crawform".into()));
    host.open_form(a_form("second"), Some("a/second.crawform".into()));
    host.open_form(a_form("third"), Some("a/third.crawform".into()));

    host.bring_to_front(1);
    host.close_form(1);
    assert_eq!(
        host.doc().map(|d| d.ir().root.as_str()),
        Some("third"),
        "closing the middle tab did not move to the right"
    );

    // Now at the end of the strip: there is no right, so the left comes forward.
    host.bring_to_front(1);
    host.close_form(1);
    assert_eq!(
        host.doc().map(|d| d.ir().root.as_str()),
        Some("first"),
        "closing the last tab did not fall back to the left"
    );

    host.close_form(0);
    assert_eq!(
        host.front(),
        None,
        "an empty strip still claims a front tab"
    );
    assert!(
        host.doc().is_none(),
        "`doc()` answered for a strip with nothing in it"
    );
}

/// A stale click from a strip that has since shrunk changes nothing.
#[test]
fn an_index_past_the_end_is_refused_without_moving_the_front() {
    let mut host = HostModel::new();
    host.open_form(a_form("only"), None);

    assert!(!host.bring_to_front(7), "a seventh tab was accepted");
    assert!(!host.close_form(7), "a seventh tab was closed");
    assert_eq!(host.front(), Some(0), "the front moved anyway");
    assert_eq!(host.forms().len(), 1);
}

/// `set_doc` keeps meaning what it meant when there could only be one form,
/// which is why its hundred callers did not have to change.
#[test]
fn set_doc_still_replaces_what_is_in_front() {
    let mut host = HostModel::new();
    host.set_doc(Some(a_form("alone")));
    assert_eq!(
        host.forms().len(),
        1,
        "set_doc on an empty strip must open one"
    );
    assert_eq!(host.doc().map(|d| d.ir().root.as_str()), Some("alone"));

    host.set_doc(Some(a_form("replaced")));
    assert_eq!(
        host.forms().len(),
        1,
        "set_doc opened a second tab instead of replacing"
    );
    assert_eq!(host.doc().map(|d| d.ir().root.as_str()), Some("replaced"));

    host.set_doc(None);
    assert!(host.doc().is_none());
    assert!(
        host.forms().is_empty(),
        "`None` must leave nothing in front"
    );
}

/// **Each form remembers its OWN saved state.**
///
/// The shell held one `rev_salva` beside its one document. With a tab each, a
/// single field would have told the person their saved form was modified the
/// moment the other one changed — the badge lying about work they never touched.
#[test]
fn saving_one_form_does_not_mark_the_other_as_modified() {
    let mut host = HostModel::new();
    host.open_form(a_form("first"), Some("a/first.crawform".into()));
    host.set_saved_revision(10);
    host.open_form(a_form("second"), Some("a/second.crawform".into()));
    host.set_saved_revision(99);

    host.bring_to_front(0);
    assert_eq!(
        host.saved_revision(),
        10,
        "the first form took the second form's saved revision"
    );

    host.bring_to_front(1);
    assert_eq!(host.saved_revision(), 99, "and the second lost its own");
}

/// The same, for the rectangle the anchor pass compares against: resizing one
/// form must not make the other believe it was resized too.
#[test]
fn each_form_keeps_its_own_previous_rectangle() {
    use crawrad_ir::Rect;
    let mut host = HostModel::new();
    host.open_form(a_form("first"), Some("a/first.crawform".into()));
    host.set_previous_rect(Some(Rect::new(0.0, 0.0, 640.0, 480.0)));
    host.open_form(a_form("second"), Some("a/second.crawform".into()));
    host.set_previous_rect(Some(Rect::new(0.0, 0.0, 200.0, 100.0)));

    host.bring_to_front(0);
    assert_eq!(
        host.previous_rect().map(|r| r.w),
        Some(640.0),
        "the first form was given the second's rectangle"
    );
}

/// With nothing open there is no form to record against, and recording anyway
/// would store state belonging to no one.
#[test]
fn recording_against_an_empty_strip_changes_nothing() {
    let mut host = HostModel::new();
    host.set_saved_revision(42);
    assert_eq!(host.saved_revision(), 0);
    assert!(host.forms().is_empty(), "a form was invented to hold it");
}

#[test]
fn the_strip_lists_the_forms_before_the_loose_files() {
    let mut host = HostModel::new();
    host.open_form(a_form("form_main"), Some(PathBuf::from("/p/form_main.rs")));
    host.open_form(
        a_form("form_other"),
        Some(PathBuf::from("/p/form_other.rs")),
    );

    let rows = host.strip(&[("main.rs".to_string(), false), ("lib.rs".to_string(), true)]);

    assert_eq!(rows.len(), 4);
    assert!(rows[0].is_form && rows[1].is_form, "the forms come first");
    assert!(!rows[2].is_form && !rows[3].is_form, "the files come after");
    assert!(rows[0].label.ends_with("form_main"));
    assert_eq!(rows[2].label, "main.rs");
    assert!(
        rows[3].dirty,
        "a loose file's unsaved mark is carried through"
    );
}

#[test]
fn a_form_with_unsaved_work_marks_its_own_tab_only() {
    let mut host = HostModel::new();
    host.open_form(a_form("form_main"), None);
    host.open_form(a_form("form_other"), None);
    host.set_saved_revision(host.doc().unwrap().revision());

    // Only the form in front is edited, by a real transaction: the revision moves
    // PAST the saved one, which is the state a person creates by typing.
    host.doc_mut()
        .unwrap()
        .apply_one(
            "move it",
            crawrad_doc::Command::SetRect {
                node: "form_other".into(),
                rect: crawrad_ir::Rect::new(8.0, 8.0, 96.0, 32.0),
            },
        )
        .unwrap();

    let rows = host.strip(&[]);
    assert!(rows[1].dirty, "the edited form shows unsaved work");
    assert!(
        !rows[0].dirty,
        "the other form was never touched; its tab must not claim it was"
    );
}

#[test]
fn a_click_past_the_forms_is_the_file_counted_from_zero() {
    let mut host = HostModel::new();
    host.open_form(a_form("form_main"), None);
    host.open_form(a_form("form_other"), None);

    assert_eq!(host.strip_target(0), StripClick::Form(0));
    assert_eq!(host.strip_target(1), StripClick::Form(1));
    // The shell's own tab list starts at zero again, past the forms.
    assert_eq!(host.strip_target(2), StripClick::File(0));
    assert_eq!(host.strip_target(3), StripClick::File(1));
}

#[test]
fn with_the_editor_up_the_lit_row_is_the_files_not_the_forms() {
    let mut host = HostModel::new();
    host.open_form(a_form("form_main"), None);
    host.open_form(a_form("form_other"), None);

    host.set_center(ViewCenter::Design);
    assert_eq!(
        host.strip_front(Some(0)),
        Some(1),
        "the form in front is lit"
    );

    host.set_center(ViewCenter::Code);
    assert_eq!(
        host.strip_front(Some(0)),
        Some(2),
        "the editor is up, so the lit row is the file's, past both forms"
    );
}

#[test]
fn a_form_without_a_path_is_named_after_its_document() {
    let mut host = HostModel::new();
    host.open_form(a_form("form_unsaved"), None);

    let rows = host.strip(&[]);
    assert!(
        rows[0].label.ends_with("form_unsaved"),
        "a form never saved still reads as itself, not as an empty tab: {}",
        rows[0].label
    );
}
