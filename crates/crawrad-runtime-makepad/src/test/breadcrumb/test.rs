use super::*;

#[test]
fn breadcrumb_action_cast() {
    let a = CrawradBreadcrumbAction::Clicked(3);
    if let CrawradBreadcrumbAction::Clicked(idx) = a {
        assert_eq!(idx, 3);
    } else {
        panic!("expected Clicked(3)");
    }
}

#[test]
fn test_breadcrumb_layout_pure() {
    let widths = [40.0, 50.0, 30.0];
    let positions = breadcrumb_layout(&widths, 8.0, 6.0);
    // advance = w + 8.0 + 12.0 = w + 20.0
    // pos[0] = 0.0
    // pos[1] = 60.0
    // pos[2] = 130.0
    assert_eq!(positions, vec![0.0, 60.0, 130.0]);
}

#[test]
fn test_breadcrumb_layout_empty() {
    let positions = breadcrumb_layout(&[], 8.0, 6.0);
    assert!(positions.is_empty());
}

#[test]
fn test_chevron_layout_empty() {
    let positions = chevron_layout(&[], CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
    assert!(positions.is_empty());
}

#[test]
fn test_chevron_layout_single() {
    let widths = [40.0];
    let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
    assert_eq!(positions.len(), 1);
    // First block has straight left edge: pad = 14.0, right pad = 20.0 => width = 74.0, x = 0.0
    assert_eq!(positions[0], (0.0, 74.0));
}

#[test]
fn test_chevron_layout_widths_and_overlap() {
    let widths = [40.0, 50.0, 30.0];
    let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
    assert_eq!(positions.len(), 3);
    // Segment 0 (first block straight): x = 0.0, w = 14.0 + 40.0 + 20.0 = 74.0
    // Segment 1 (notched left): x = 74.0 - 8.0 = 66.0, w = 24.0 + 50.0 + 20.0 = 94.0
    // Segment 2 (notched left): x = 66.0 + 94.0 - 8.0 = 152.0, w = 24.0 + 30.0 + 20.0 = 74.0
    assert_eq!(positions[0], (0.0, 74.0));
    assert_eq!(positions[1], (66.0, 94.0));
    assert_eq!(positions[2], (152.0, 74.0));
}
