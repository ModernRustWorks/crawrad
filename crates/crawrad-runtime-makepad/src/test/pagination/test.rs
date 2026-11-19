use super::*;

#[test]
fn pagination_action_cast() {
    let a = CrawradPaginationAction::Changed(3);
    if let CrawradPaginationAction::Changed(idx) = a {
        assert_eq!(idx, 3);
    } else {
        panic!("expected Changed(3)");
    }
}

#[test]
fn test_pagination_slots_example_1() {
    let slots = pagination_slots(20, 0, 7, false);
    assert_eq!(
        slots,
        vec![
            PageSlot::Page(0),
            PageSlot::Page(1),
            PageSlot::Page(2),
            PageSlot::Page(3),
            PageSlot::Page(4),
            PageSlot::Gap,
            PageSlot::Page(19),
        ]
    );
}

#[test]
fn test_pagination_slots_example_2() {
    let slots = pagination_slots(20, 9, 7, false);
    assert_eq!(
        slots,
        vec![
            PageSlot::Page(0),
            PageSlot::Gap,
            PageSlot::Page(8),
            PageSlot::Page(9),
            PageSlot::Page(10),
            PageSlot::Gap,
            PageSlot::Page(19),
        ]
    );
}

#[test]
fn test_pagination_slots_example_3() {
    let slots = pagination_slots(20, 19, 7, false);
    assert_eq!(
        slots,
        vec![
            PageSlot::Page(0),
            PageSlot::Gap,
            PageSlot::Page(15),
            PageSlot::Page(16),
            PageSlot::Page(17),
            PageSlot::Page(18),
            PageSlot::Page(19),
        ]
    );
}

#[test]
fn test_pagination_slots_count_le_max() {
    let slots = pagination_slots(5, 2, 7, false);
    assert_eq!(
        slots,
        vec![
            PageSlot::Page(0),
            PageSlot::Page(1),
            PageSlot::Page(2),
            PageSlot::Page(3),
            PageSlot::Page(4),
        ]
    );
}

#[test]
fn test_pagination_slots_count_1() {
    let slots = pagination_slots(1, 0, 7, true);
    assert_eq!(
        slots,
        vec![PageSlot::Prev, PageSlot::Page(0), PageSlot::Next,]
    );
}

#[test]
fn test_pagination_slots_clamped() {
    let slots = pagination_slots(10, 50, 7, false);
    assert_eq!(
        slots,
        vec![
            PageSlot::Page(0),
            PageSlot::Gap,
            PageSlot::Page(5),
            PageSlot::Page(6),
            PageSlot::Page(7),
            PageSlot::Page(8),
            PageSlot::Page(9),
        ]
    );
}
