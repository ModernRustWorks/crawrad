//! The session's numbered bookmarks and their reconciliation with the view.

use super::{BookmarkSync, Bookmarks};
use std::path::Path;

/// **Per file, by number; only 1 to 9.**
#[test]
fn bookmarks_are_kept_per_file_and_number() {
    let mut b = Bookmarks::default();
    let main = Path::new("src/main.rs");
    let form = Path::new("src/form_main.rs");
    b.set(main, &[(3, 40), (1, 10), (0, 5), (12, 9)]);
    b.set(form, &[(1, 2)]);
    assert_eq!(
        b.of(main),
        vec![(1, 10), (3, 40)],
        "0 and 12 are not numbers"
    );
    assert_eq!(b.line_of(main, 3), Some(40));
    assert_eq!(b.line_of(form, 1), Some(2));
    assert_eq!(b.line_of(form, 3), None);
    b.set(form, &[]);
    assert!(b.of(form).is_empty());
}

/// **A view of another file is given this file's; a view of this file that
/// changed wins; one that agrees changes nothing.**
#[test]
fn the_view_is_given_or_taken_from() {
    let mut b = Bookmarks::default();
    let main = Path::new("src/main.rs");
    let form = Path::new("src/form_main.rs");
    b.set(main, &[(1, 10)]);
    assert_eq!(
        b.sync_view(main, Some(form), &[(2, 7)]),
        BookmarkSync::Push(vec![(1, 10)])
    );
    assert_eq!(
        b.sync_view(main, None, &[]),
        BookmarkSync::Push(vec![(1, 10)])
    );
    assert_eq!(
        b.sync_view(main, Some(main), &[(1, 10)]),
        BookmarkSync::Same
    );
    // An Enter above moved it; a key set number 2.
    assert_eq!(
        b.sync_view(main, Some(main), &[(2, 3), (1, 11)]),
        BookmarkSync::Pulled
    );
    assert_eq!(b.of(main), vec![(1, 11), (2, 3)]);
}
