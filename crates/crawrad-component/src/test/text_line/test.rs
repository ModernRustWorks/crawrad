//! Where a single line of text goes.

use super::{CHEVRON_OVERLAP, CHEVRON_PADS, CHEVRON_TIP, cap_centred_top, chevron_layout, font_px};

/// The Chips measure that gave the rule: a 24 px chip at 11 pt.
#[test]
fn the_chip_measure() {
    let top = cap_centred_top(24.0, font_px(11.0));
    assert!((top - 3.685).abs() < 0.01, "{top}");
}

/// The capitals' middle is the box's middle, at any size.
#[test]
fn the_capitals_are_centred() {
    for (h, pt) in [(24.0, 10.0), (32.0, 14.0), (20.0, 8.0)] {
        let px = font_px(pt);
        let caps_middle = cap_centred_top(h, px) + 0.227 * px + 0.68 * px / 2.0;
        assert!(
            (caps_middle - h / 2.0).abs() < 0.01,
            "{h} {pt}: {caps_middle}"
        );
    }
}

#[test]
fn test_chevron_layout_empty() {
    let positions = chevron_layout(&[], CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
    assert!(positions.is_empty());
}

#[test]
fn test_chevron_layout_single_segment() {
    let widths = [40.0];
    let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
    assert_eq!(positions.len(), 1);
    // Block 0: left_pad = 14.0, w = 40.0, right_pad = 20.0 => width = 74.0, x = 0.0
    assert_eq!(positions[0], (0.0, 74.0));
}

#[test]
fn test_chevron_layout_multiple_segments() {
    let widths = [40.0, 50.0, 30.0];
    let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, CHEVRON_PADS);
    assert_eq!(positions.len(), 3);
    // Segment 0: x = 0.0, width = 14.0 + 40.0 + 20.0 = 74.0
    // Segment 1: x = 0.0 + 74.0 - 8.0 = 66.0, width = 24.0 + 50.0 + 20.0 = 94.0
    // Segment 2: x = 66.0 + 94.0 - 8.0 = 152.0, width = 24.0 + 30.0 + 20.0 = 74.0
    assert_eq!(positions[0], (0.0, 74.0));
    assert_eq!(positions[1], (66.0, 94.0));
    assert_eq!(positions[2], (152.0, 74.0));
}
