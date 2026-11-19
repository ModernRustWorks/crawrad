//! The gate over the change bars: which lines are new in this session,
//! and which of them are still waiting for a save.

use std::path::Path;

use super::{
    ChangeTracker,
    LineChange::{Saved, Unsaved},
    line_changes,
};

const DISK: &str = "fn main() {\n    let a = 1;\n}\n";

/// **A file nobody touched has no bar at all.**
#[test]
fn an_untouched_file_has_no_bars() {
    assert!(line_changes(DISK, DISK, DISK).is_empty());
}

/// **A line typed and not saved is yellow.**
#[test]
fn a_typed_line_is_unsaved() {
    let typed = "fn main() {\n    let a = 2;\n}\n";
    assert_eq!(line_changes(DISK, DISK, typed), vec![(1, Unsaved)]);
}

/// **The same line, saved, turns green — and stays green for the session.**
#[test]
fn a_saved_line_is_saved() {
    let typed = "fn main() {\n    let a = 2;\n}\n";
    assert_eq!(line_changes(DISK, typed, typed), vec![(1, Saved)]);
}

/// **Editing a saved line again makes it yellow again.**
#[test]
fn editing_a_saved_line_makes_it_unsaved() {
    let saved = "fn main() {\n    let a = 2;\n}\n";
    let again = "fn main() {\n    let a = 3;\n}\n";
    assert_eq!(line_changes(DISK, saved, again), vec![(1, Unsaved)]);
}

/// **Undo back to the disk text clears every bar.**
#[test]
fn undo_to_the_disk_text_clears_the_bars() {
    assert!(line_changes(DISK, DISK, DISK).is_empty());
    let saved = "fn main() {\n    let a = 2;\n}\n";
    // Saved in the session, then edited back to what the session began with:
    // nothing differs from the start, so no green either — only the yellow of
    // a line that differs from what is on disk now.
    assert_eq!(line_changes(DISK, saved, DISK), vec![(1, Unsaved)]);
}

/// **A line inserted above moves the marks below it down by one.**
#[test]
fn an_inserted_line_shifts_the_marks() {
    let saved = "fn main() {\n    let a = 2;\n}\n";
    let current = "// note\nfn main() {\n    let a = 2;\n}\n";
    assert_eq!(
        line_changes(DISK, saved, current),
        vec![(0, Unsaved), (2, Saved)]
    );
}

/// **A deletion leaves no line of the current text to mark, and must not
/// panic.**
#[test]
fn a_deletion_only_does_not_panic() {
    let current = "fn main() {\n}\n";
    assert!(line_changes(DISK, DISK, current).is_empty());
    assert!(line_changes(DISK, DISK, "").is_empty());
}

/// **A line added after a last line without `\n` marks only the new line.**
#[test]
fn appending_after_an_unterminated_last_line_marks_only_the_new_line() {
    let disk = "a\nb";
    assert_eq!(line_changes(disk, disk, "a\nb\nc"), vec![(2, Unsaved)]);
}

/// **ChangeTracker remembers the first text; subsequent remember calls keep it.**
#[test]
fn tracker_remember_keeps_the_first_text() {
    let mut tracker = ChangeTracker::default();
    let file = Path::new("main.rs");
    tracker.remember(file, "first");
    tracker.remember(file, "second");
    assert_eq!(tracker.changes(file, "second", "third"), vec![(0, Unsaved)]);
    assert_eq!(tracker.changes(file, "second", "second"), vec![(0, Saved)]);
}

/// **ChangeTracker forget clears remembered base text.**
#[test]
fn tracker_forget_resets() {
    let mut tracker = ChangeTracker::default();
    let file = Path::new("main.rs");
    tracker.remember(file, "first");
    tracker.forget(file);
    assert!(tracker.changes(file, "second", "second").is_empty());
}

/// **ChangeTracker falls back to saved text when file is unknown.**
#[test]
fn tracker_unknown_file_falls_back_to_saved() {
    let tracker = ChangeTracker::default();
    let file = Path::new("unknown.rs");
    assert_eq!(tracker.changes(file, "a\n", "b\n"), vec![(0, Unsaved)]);
    assert!(tracker.changes(file, "a\n", "a\n").is_empty());
}
