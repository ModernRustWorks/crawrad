use super::{click, compare, matching, settle, sort, sorted_order};

#[test]
fn a_plain_click_marks_only_its_row() {
    let mut s = vec![1, 2, 5];
    click(&mut s, Some(5), 3, true, false, false);
    assert_eq!(s, vec![3]);
}

#[test]
fn ctrl_toggles_a_row_in_and_out_keeping_the_order() {
    let mut s = vec![1, 5];
    click(&mut s, Some(5), 3, true, true, false);
    assert_eq!(s, vec![1, 3, 5]);
    click(&mut s, Some(3), 1, true, true, false);
    assert_eq!(s, vec![3, 5]);
}

#[test]
fn shift_marks_the_run_from_the_row_in_focus_either_way() {
    let mut s = vec![2];
    click(&mut s, Some(2), 5, true, false, true);
    assert_eq!(s, vec![2, 3, 4, 5]);
    click(&mut s, Some(5), 1, true, false, true);
    assert_eq!(s, vec![1, 2, 3, 4, 5]);
    let mut s = Vec::new();
    click(&mut s, None, 4, true, false, true);
    assert_eq!(s, vec![4], "with no row in focus the run is the row itself");
}

#[test]
fn without_multi_select_the_keys_change_nothing() {
    let mut s = vec![1];
    click(&mut s, Some(1), 4, false, true, false);
    assert_eq!(s, vec![4]);
    click(&mut s, Some(4), 2, false, false, true);
    assert_eq!(s, vec![2]);
}

#[test]
fn settle_follows_selected_when_single_and_drops_rows_past_the_end() {
    let mut s = vec![0, 1];
    settle(&mut s, Some(3), 10, false);
    assert_eq!(s, vec![3], "single: the mark is where `selected` is");
    settle(&mut s, None, 10, false);
    assert!(s.is_empty());

    let mut s = vec![1, 4, 7];
    settle(&mut s, Some(1), 5, true);
    assert_eq!(s, vec![1, 4], "multi: kept, but not past the end");
}

#[test]
fn sorted_is_alphabetical_ignoring_case() {
    assert_eq!(compare("apple", "Banana"), std::cmp::Ordering::Less);
    assert_eq!(compare("Zeta", "alpha"), std::cmp::Ordering::Greater);
    assert_eq!(
        sorted_order(&["b", "A", "c", "a"]),
        vec![1, 3, 0, 2],
        "equal texts keep their order"
    );
}

#[test]
fn sorting_moves_the_marked_rows_with_their_text() {
    let mut items: Vec<String> = ["Pear", "apple", "Fig"].map(String::from).to_vec();
    let mut selected = Some(0);
    let mut selection = vec![0, 2];
    sort(&mut items, &mut selected, &mut selection);
    assert_eq!(items, ["apple", "Fig", "Pear"]);
    assert_eq!(selected, Some(2), "Pear is still the row in focus");
    assert_eq!(selection, vec![1, 2], "Pear and Fig are still marked");
}

#[test]
fn a_list_already_in_order_is_left_alone() {
    let mut items: Vec<String> = ["a", "B", "c"].map(String::from).to_vec();
    let mut selected = Some(1);
    let mut selection = vec![1];
    sort(&mut items, &mut selected, &mut selection);
    assert_eq!(items, ["a", "B", "c"]);
    assert_eq!((selected, selection), (Some(1), vec![1]));
}

#[test]
fn matching_filters_by_case_insensitive_prefix() {
    let items = vec![
        "Apple".to_string(),
        "Apricot".to_string(),
        "Banana".to_string(),
        "application".to_string(),
        "Cherry".to_string(),
    ];
    assert_eq!(matching(&items, ""), vec![0, 1, 2, 3, 4]);
    assert_eq!(matching(&items, "ap"), vec![0, 1, 3]);
    assert_eq!(matching(&items, "APP"), vec![0, 3]);
    assert_eq!(matching(&items, "b"), vec![2]);
    assert_eq!(matching(&items, "xyz"), Vec::<usize>::new());
}
