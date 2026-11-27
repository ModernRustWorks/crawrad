use super::*;

#[test]
fn toggling_adds_and_removes_and_an_empty_file_leaves_no_entry() {
    let mut bp = Breakpoints::default();
    let file = Path::new("src/form_main.rs");
    assert!(bp.toggle(file, 12));
    assert!(bp.toggle(file, 3));
    assert_eq!(bp.lines(file), vec![3, 12]);
    assert_eq!(bp.count(), 2);
    assert!(!bp.toggle(file, 12));
    assert!(!bp.toggle(file, 3));
    assert!(bp.is_empty());
    assert_eq!(bp.files().count(), 0);
}

#[test]
fn set_lines_replaces_one_file_and_leaves_the_others() {
    let mut bp = Breakpoints::default();
    let a = Path::new("a.rs");
    let b = Path::new("b.rs");
    bp.set_lines(a, [5, 1]);
    bp.set_lines(b, [7]);
    bp.set_lines(a, [2, 0, 2]);
    assert_eq!(bp.lines(a), vec![2]);
    assert_eq!(bp.lines(b), vec![7]);
    bp.set_lines(a, []);
    assert_eq!(
        bp.files().map(|(f, _)| f.to_path_buf()).collect::<Vec<_>>(),
        vec![PathBuf::from("b.rs")]
    );
    // There is no line zero.
    assert!(!bp.toggle(a, 0));
    assert!(bp.lines(a).is_empty());
}

#[test]
fn breakpoints_set_from_outside_are_pushed_to_the_view_not_overwritten() {
    let mut bp = Breakpoints::default();
    let form = Path::new("src/form_main.rs");
    bp.set_lines(form, [3]);
    assert_eq!(bp.sync_view(form, Some(form), &[2]), ViewSync::Same);
    // The MCP adds line 12 while the view still shows only line 3.
    bp.set_lines_from_outside(form, [3, 12]);
    assert_eq!(
        bp.sync_view(form, Some(form), &[2]),
        ViewSync::Push(vec![2, 11])
    );
    assert_eq!(bp.lines(form), vec![3, 12]);
    // Once pushed, the view is the editor's again.
    assert_eq!(bp.sync_view(form, Some(form), &[2, 11]), ViewSync::Same);
}

#[test]
fn one_file_spelled_two_ways_is_one_file() {
    let dir = std::env::temp_dir().join(format!("crawrad-bp-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("src")).unwrap();
    let file = dir.join("src").join("main.rs");
    std::fs::write(&file, "fn main() {}\n").unwrap();
    let other_spelling = dir.join("src").join("..").join("src").join("main.rs");
    let mut bp = Breakpoints::default();
    bp.toggle(&file, 1);
    assert_eq!(bp.lines(&other_spelling), vec![1]);
    let key = file_key(&file);
    assert!(!key.to_string_lossy().starts_with(r"\\?\"), "{key:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_view_of_another_file_is_given_this_file_s_lines_zero_based() {
    let mut bp = Breakpoints::default();
    let form = Path::new("src/form_main.rs");
    bp.set_lines(form, [12, 3]);
    assert_eq!(bp.sync_view(form, None, &[]), ViewSync::Push(vec![2, 11]));
    assert_eq!(
        bp.sync_view(form, Some(Path::new("src/main.rs")), &[0]),
        ViewSync::Push(vec![2, 11])
    );
    // Pushing does not touch the session.
    assert_eq!(bp.lines(form), vec![3, 12]);
}

#[test]
fn a_view_that_was_clicked_or_edited_wins_and_one_that_agrees_changes_nothing() {
    let mut bp = Breakpoints::default();
    let form = Path::new("src/form_main.rs");
    bp.set_lines(form, [3, 12]);
    assert_eq!(bp.sync_view(form, Some(form), &[2, 11]), ViewSync::Same);
    // An Enter above line 12 moved it to 13; a click added line 20.
    assert_eq!(
        bp.sync_view(form, Some(form), &[2, 12, 19]),
        ViewSync::Pulled
    );
    assert_eq!(bp.lines(form), vec![3, 13, 20]);
    // Every dot clicked away empties the file.
    assert_eq!(bp.sync_view(form, Some(form), &[]), ViewSync::Pulled);
    assert!(bp.is_empty());
}

#[test]
fn a_logpoint_is_a_breakpoint_line_with_a_message() {
    let mut bp = Breakpoints::default();
    let f = Path::new("src/form_main.rs");
    bp.set_lines(f, [3]);
    bp.set_logpoint(f, 12, "total = {total}");
    assert_eq!(bp.lines(f), vec![3, 12]);
    assert_eq!(bp.log_message(f, 12), Some("total = {total}"));
    assert_eq!(bp.log_message(f, 3), None);
    let sources = bp.sources(f);
    assert_eq!(sources[0].log_message, None);
    assert_eq!(sources[1].log_message.as_deref(), Some("total = {total}"));
    // An empty message makes it a plain breakpoint again.
    bp.set_logpoint(f, 12, "  ");
    assert_eq!(bp.log_message(f, 12), None);
    assert_eq!(bp.lines(f), vec![3, 12]);
}

#[test]
fn the_message_follows_its_line_and_goes_with_it() {
    let mut bp = Breakpoints::default();
    let f = Path::new("src/form_main.rs");
    bp.set_lines(f, [3]);
    bp.set_logpoint(f, 12, "x = {x}");
    // An Enter above both moves both down by one.
    bp.set_lines(f, [4, 13]);
    assert_eq!(bp.log_message(f, 13), Some("x = {x}"));
    assert_eq!(bp.log_message(f, 12), None);
    // A click on the dot removes the line and its message.
    assert!(!bp.toggle(f, 13));
    assert_eq!(bp.log_message(f, 13), None);
    assert!(!bp.toggle(f, 4));
    assert!(bp.is_empty());
}
