use crate::{CHEVRON_OVERLAP, CHEVRON_TIP, ChevronPads, breadcrumb_layout, chevron_layout};

#[test]
fn test_breadcrumb_layout_empty() {
    let positions = breadcrumb_layout(&[], 8.0, 6.0);
    assert!(positions.is_empty());
}

#[test]
fn test_breadcrumb_layout_single() {
    let widths = [40.0];
    let positions = breadcrumb_layout(&widths, 8.0, 6.0);
    assert_eq!(positions, vec![0.0]);
}

#[test]
fn test_breadcrumb_layout_multiple() {
    let widths = [40.0, 50.0, 60.0];
    // sep = 8.0, gap = 6.0 => advance = w + 8.0 + 12.0 = w + 20.0
    // pos[0] = 0.0
    // pos[1] = 0.0 + 40.0 + 20.0 = 60.0
    // pos[2] = 60.0 + 50.0 + 20.0 = 130.0
    let positions = breadcrumb_layout(&widths, 8.0, 6.0);
    assert_eq!(positions, vec![0.0, 60.0, 130.0]);
}

#[test]
fn test_chevron_layout_empty() {
    let positions = chevron_layout(&[], CHEVRON_TIP, CHEVRON_OVERLAP, ChevronPads::default());
    assert!(positions.is_empty());
}

#[test]
fn test_chevron_layout_single() {
    let widths = [40.0];
    let pads = ChevronPads::default();
    let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, pads);
    assert_eq!(positions.len(), 1);
    let (x, w) = positions[0];
    assert_eq!(x, 0.0);
    assert_eq!(w, 14.0 + 40.0 + 20.0 + 12.0);
}

#[test]
fn test_chevron_layout_multiple_overlap() {
    let widths = [40.0, 50.0];
    let pads = ChevronPads::default();
    let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, pads);
    assert_eq!(positions.len(), 2);
    let (x0, w0) = positions[0];
    let (x1, w1) = positions[1];
    assert_eq!(x0, 0.0);
    assert_eq!(w0, 86.0);
    assert_eq!(x1, 78.0);
    assert_eq!(w1, 106.0);
}
