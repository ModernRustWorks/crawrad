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
