use super::*;

#[test]
fn chips_action_cast() {
    let a = CrawradChipsAction::Selected(5);
    if let CrawradChipsAction::Selected(idx) = a {
        assert_eq!(idx, 5);
    } else {
        panic!("expected Selected(5)");
    }

    let r = CrawradChipsAction::Removed(2);
    if let CrawradChipsAction::Removed(idx) = r {
        assert_eq!(idx, 2);
    } else {
        panic!("expected Removed(2)");
    }
}

#[test]
fn test_chips_layout_pure() {
    let widths = [40.0, 50.0, 30.0];
    let positions = chips_layout(&widths, 200.0, 24.0, 6.0);
    assert_eq!(positions, vec![(0.0, 0.0), (46.0, 0.0), (102.0, 0.0),]);
}

#[test]
fn test_chips_layout_wrapping() {
    let widths = [60.0, 60.0, 60.0];
    let positions = chips_layout(&widths, 100.0, 24.0, 6.0);
    assert_eq!(positions, vec![(0.0, 0.0), (0.0, 30.0), (0.0, 60.0),]);
}

/// Where each chip and its `×` are, in the widget's coordinates:
/// the click and MCP read the same answer.
#[test]
fn chip_rects_place_the_chips_and_their_remove_buttons() {
    let area = Rect {
        pos: dvec2(100.0, 50.0),
        size: dvec2(200.0, 60.0),
    };
    let items = vec![
        "Linux".to_string(),
        "Windows 64".to_string(),
        "mac".to_string(),
    ];
    let r = chip_rects(&items, true, 0.0, area);
    // "Linux": 5 chars × 7 + 2 × 8 + 14 = 65 wide, at the area's corner
    assert_eq!(r[0].1.pos, dvec2(100.0, 50.0));
    assert_eq!(r[0].1.size.x, 65.0);
    let x = r[0].2.expect("removable");
    assert_eq!(x.pos.x, 100.0 + 65.0 - 14.0 - 8.0);
    // the third does not fit the first line (65 + 6 + 100 + 6 + 51 > 200): next line
    assert_eq!(r[2].1.pos.y, 50.0 + 24.0 + 6.0);
    // not removable: no `×`
    assert!(
        chip_rects(&items, false, 0.0, area)
            .iter()
            .all(|c| c.2.is_none())
    );
    // a line that does not fit the height is not reported
    let short = Rect {
        pos: area.pos,
        size: dvec2(200.0, 24.0),
    };
    assert_eq!(chip_rects(&items, true, 0.0, short).len(), 2);
}

#[test]
fn palette_widgets_are_born_enabled() {
    // Previously `enabled` defaulted to false, so a host that never called
    // `set_enabled` (the IDE's Build dialog) had every click ignored.
    for (name, src) in [
        ("chips", include_str!("../../chips.rs")),
        ("breadcrumb", include_str!("../../breadcrumb.rs")),
        ("multi_select", include_str!("../../multi_select.rs")),
        ("pagination", include_str!("../../pagination.rs")),
        ("check_list", include_str!("../../check_list.rs")),
    ] {
        assert!(
            src.contains("#[rust(true)]\n    enabled: bool,"),
            "{name} must be born enabled"
        );
        // Its setters come from the shared glue, not a copy
        assert!(
            src.contains("Ref, enabled);"),
            "{name} must use palette_glue!(…, enabled)"
        );
        assert!(
            !src.contains("pub fn set_enabled"),
            "{name} keeps its own set_enabled"
        );
    }
}
