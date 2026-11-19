use super::*;

#[test]
fn segment_kind_as_str_returns_stable_identifiers() {
    assert_eq!(SegmentKind::Text.as_str(), "text");
    assert_eq!(SegmentKind::Block.as_str(), "block");
}

#[test]
fn an_unbound_segment_list_has_no_items_and_reports_nothing() {
    let list = CrawradSegmentListRef::default();
    assert!(list.borrow().is_none());
    assert!(list.item(0).is_empty());
}
