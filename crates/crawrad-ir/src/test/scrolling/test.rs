//! The bar rule, with the owner's two-minute check inside it.

use super::*;

/// **An empty box has no bar.** This is the case the owner saw broken.
///
/// The designer drew both bars on a `ScrollBox` without a single child, and
/// the generated application drew none. The designer was the wrong one: in
/// a classic RAD IDE an empty `TScrollBox` shows no bar.
#[test]
fn an_empty_box_has_no_bar() {
    let content = content_extent((10.0, 20.0), std::iter::empty());
    assert_eq!(content, (0.0, 0.0));
    assert_eq!(bars_needed((200.0, 150.0), content), Bars::default());
}

/// **The two-minute check**, exactly as it is written there: a
/// 300x150 `TScrollBox` with a child at `Top := 200`.
///
/// > *"A barra vertical aparece no designer na hora."*
#[test]
fn a_child_that_overflows_brings_the_vertical_bar() {
    // The box at (10, 20) on the form, 300x150; the child at `Top := 200`
    // INSIDE it, that is at y = 20 + 200 on the form.
    let box_origin = (10.0, 20.0);
    let child = (10.0 + 8.0, 20.0 + 200.0, 75.0, 25.0);
    let content = content_extent(box_origin, [child].into_iter());
    let bars = bars_needed((300.0, 150.0), content);
    assert!(
        bars.vertical,
        "the child starts at 200 and reaches 225: past 150, so the bar must come"
    );
    assert!(
        !bars.horizontal,
        "and the horizontal one must NOT: 83 fits in 300. A bar with nothing \
         to scroll is the one a classic RAD IDE does not show"
    );
}

/// **Touching the edge is fitting.** `>` and not `>=`.
#[test]
fn content_ending_on_the_edge_fits() {
    let content = content_extent((0.0, 0.0), [(0.0, 0.0, 200.0, 150.0)].into_iter());
    assert_eq!(content, (200.0, 150.0));
    assert_eq!(
        bars_needed((200.0, 150.0), content),
        Bars::default(),
        "content ending exactly on the edge has not one pixel to scroll, and a \
         bar that scrolls zero is noise"
    );
}

/// **One pixel over already asks for the bar.**
#[test]
fn one_pixel_over_already_asks_for_the_bar() {
    let content = content_extent((0.0, 0.0), [(0.0, 0.0, 201.0, 150.0)].into_iter());
    let bars = bars_needed((200.0, 150.0), content);
    assert!(bars.horizontal && !bars.vertical);
}

/// **A child above or to the left of the origin extends nothing.**
///
/// In a classic RAD IDE a `ScrollBox` scrolls forward: the bar does not run back past
/// where the content starts. Without this floor, a child at a negative
/// coordinate would make the extent negative and the arithmetic meaningless.
#[test]
fn a_child_before_the_origin_extends_nothing() {
    let content = content_extent((100.0, 100.0), [(20.0, 20.0, 30.0, 30.0)].into_iter());
    assert_eq!(content, (0.0, 0.0));
    assert_eq!(bars_needed((200.0, 150.0), content), Bars::default());
}

/// **The FURTHEST child decides**, not the last one in the list.
#[test]
fn the_extent_is_the_furthest_child() {
    let content = content_extent(
        (0.0, 0.0),
        [
            (0.0, 0.0, 500.0, 10.0),
            (0.0, 0.0, 10.0, 10.0),
            (0.0, 400.0, 10.0, 10.0),
        ]
        .into_iter(),
    );
    assert_eq!(content, (500.0, 410.0));
    let bars = bars_needed((200.0, 150.0), content);
    assert!(bars.vertical && bars.horizontal);
}

/// **The axes do not couple**, and this test keeps the choice with its reason.
///
/// In the classic library and in egui, turning one bar on shrinks the other axis, because
/// there it eats client area. Ours sits ON TOP (ADR 0012: drawing does not
/// move what the document placed), so turning one on cannot turn the other on —
/// that would be a rule with no cause.
#[test]
fn the_axes_do_not_couple() {
    // Overflows badly in height and fits by a hair in width.
    let content = content_extent((0.0, 0.0), [(0.0, 0.0, 199.0, 900.0)].into_iter());
    let bars = bars_needed((200.0, 150.0), content);
    assert!(bars.vertical);
    assert!(
        !bars.horizontal,
        "the vertical bar turned the horizontal one on. That would only make \
         sense if the bar ate client area — and ours does not, or it would \
         push the child the user placed"
    );
}

/// `AutoScroll` off closes both axes, and each axis property closes
/// its own one.
#[test]
fn auto_scroll_and_the_axis_properties_decide_which_axes_scroll() {
    let both = Bars {
        vertical: true,
        horizontal: true,
    };
    assert_eq!(scroll_box_axes(true, true, true), both);
    assert_eq!(scroll_box_axes(false, true, true), Bars::default());
    assert_eq!(
        scroll_box_axes(true, false, true),
        Bars {
            vertical: true,
            horizontal: false,
        }
    );
    assert_eq!(
        scroll_box_axes(true, true, false),
        Bars {
            vertical: false,
            horizontal: true,
        }
    );
}

#[test]
fn turning_a_scroll_bars_kind_swaps_a_box_of_the_other_shape() {
    let wide = crate::Rect::new(4.0, 6.0, 120.0, 17.0);
    assert_eq!(
        scroll_bar_box_for_kind(wide, "sbVertical"),
        Some(crate::Rect::new(4.0, 6.0, 17.0, 120.0))
    );
    assert_eq!(scroll_bar_box_for_kind(wide, "sbHorizontal"), None);
    let tall = crate::Rect::new(0.0, 0.0, 17.0, 120.0);
    assert_eq!(scroll_bar_box_for_kind(tall, "sbVertical"), None);
    assert_eq!(
        scroll_bar_box_for_kind(tall, "sbHorizontal"),
        Some(crate::Rect::new(0.0, 0.0, 120.0, 17.0))
    );
}
