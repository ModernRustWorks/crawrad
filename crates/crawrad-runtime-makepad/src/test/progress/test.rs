//! How full a ProgressBar is drawn.

use super::fraction;

#[test]
fn the_position_is_a_fraction_of_the_range() {
    assert_eq!(fraction(0, 0, 100), 0.0);
    assert_eq!(fraction(25, 0, 100), 0.25);
    assert_eq!(fraction(100, 0, 100), 1.0);
    assert_eq!(fraction(15, 10, 20), 0.5);
}

#[test]
fn outside_the_range_is_clamped_and_an_empty_range_is_empty() {
    assert_eq!(fraction(150, 0, 100), 1.0);
    assert_eq!(fraction(-5, 0, 100), 0.0);
    assert_eq!(fraction(5, 10, 10), 0.0);
    assert_eq!(fraction(5, 10, 0), 0.0);
}

/// The designer fills the bar by the SAME fraction the app does:
/// `Position = 85` was drawn at a fixed 45 %.
#[test]
fn the_designer_fills_the_bar_like_the_app() {
    use crate::components::progress_bar::fill;
    use crawrad_ir::Value;
    use makepad_widgets::{Rect, dvec2};
    let r = Rect {
        pos: dvec2(0.0, 0.0),
        size: dvec2(102.0, 18.0),
    };
    let mut props = std::collections::BTreeMap::new();
    props.insert("position".to_string(), Value::Int(85));
    let f = fill(r, &props);
    assert_eq!(
        (f.size.x, f.size.y),
        (85.0, 16.0),
        "85 % of the 100-wide inside"
    );
    props.insert("orientation".to_string(), Value::Enum("Vertical".into()));
    props.insert("position".to_string(), Value::Int(50));
    let v = fill(r, &props);
    assert_eq!(
        (v.size.x, v.size.y, v.pos.y),
        (100.0, 8.0, 9.0),
        "half, from the bottom"
    );
}
