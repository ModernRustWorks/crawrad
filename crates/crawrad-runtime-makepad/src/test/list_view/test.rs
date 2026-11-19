#[test]
fn list_view_action_cast() {
    use super::*;
    let a = CrawradListViewAction::Selected(42);
    if let CrawradListViewAction::Selected(idx) = a {
        assert_eq!(idx, 42);
    } else {
        panic!("expected Selected(42)");
    }

    let d = CrawradListViewAction::DoubleClicked(10);
    if let CrawradListViewAction::DoubleClicked(idx) = d {
        assert_eq!(idx, 10);
    } else {
        panic!("expected DoubleClicked(10)");
    }
}

#[test]
fn list_view_item_trait() {
    use super::*;
    let item = ListViewItem {
        text: "Alpha".to_string(),
        sub_items: vec!["100 KB".to_string()],
    };
    assert_eq!(item.text(), "Alpha");
    assert_eq!(item.sub_items(), &["100 KB".to_string()]);

    let tuple = ("Beta".to_string(), vec!["200 KB".to_string()]);
    assert_eq!(tuple.text(), "Beta");
    assert_eq!(tuple.sub_items(), &["200 KB".to_string()]);
}

#[test]
fn a_double_click_is_found_behind_the_click_of_the_same_batch() {
    use super::*;
    let batch = || {
        [
            CrawradListViewAction::Selected(3),
            CrawradListViewAction::DoubleClicked(3),
            CrawradListViewAction::ColumnClicked(1),
        ]
        .into_iter()
    };
    let double = |a| match a {
        CrawradListViewAction::DoubleClicked(i) => Some(i),
        _ => None,
    };
    let column = |a| match a {
        CrawradListViewAction::ColumnClicked(i) => Some(i),
        _ => None,
    };
    assert_eq!(first_of(batch(), double), Some(3));
    assert_eq!(first_of(batch(), column), Some(1));
    assert_eq!(first_of(batch().take(1), double), None);
}
