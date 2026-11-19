//! Where the designer draws a TrackBar's value.

use super::{look_parts, parts};
use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};
use std::collections::BTreeMap;

/// The fill follows `Position`, as the app's slider: measured 7 of 0..10 on a
/// 200-wide bar filled ~142 px. It was a thumb at a fixed 40 %.
#[test]
fn the_fill_follows_the_position() {
    let r = Rect {
        pos: dvec2(40.0, 40.0),
        size: dvec2(200.0, 24.0),
    };
    let mut props = BTreeMap::new();
    props.insert("position".to_string(), Value::Int(7));
    let (track, fill, value) = parts(r, &props);
    assert_eq!(value, 7);
    assert_eq!(track.size.x, 200.0);
    assert!((fill.size.x - 140.0).abs() < 0.01, "{}", fill.size.x);
    assert_eq!(fill.pos.y, 61.0, "along the bottom of the box");
}

/// Vertical: the line runs up the middle and fills from the bottom.
#[test]
fn a_vertical_bar_fills_from_the_bottom() {
    let r = Rect {
        pos: dvec2(0.0, 0.0),
        size: dvec2(24.0, 100.0),
    };
    let mut props = BTreeMap::new();
    props.insert("orientation".to_string(), Value::Enum("Vertical".into()));
    props.insert("position".to_string(), Value::Int(5));
    let (track, fill, _) = parts(r, &props);
    assert_eq!(track.size.y, 100.0);
    assert_eq!((fill.pos.y, fill.size.y), (50.0, 50.0));
}

fn look(name: &str, position: i64) -> BTreeMap<String, Value> {
    let mut props = BTreeMap::new();
    props.insert("makepad.look".to_string(), Value::Enum(name.into()));
    props.insert("position".to_string(), Value::Int(position));
    props
}

/// `makepad.look`: `Minimal`, or no look at all, keeps the line.
#[test]
fn minimal_and_no_look_keep_the_line() {
    let r = Rect {
        pos: dvec2(0.0, 0.0),
        size: dvec2(300.0, 32.0),
    };
    assert!(look_parts(r, &look("Minimal", 4)).is_none());
    assert!(look_parts(r, &BTreeMap::new()).is_none());
}

/// `Box`: the app's `Slider`, an 11 px box along the bottom, filled through
/// its middle up to the value.
#[test]
fn the_box_look_is_an_inset_bar_along_the_bottom() {
    let r = Rect {
        pos: dvec2(20.0, 40.0),
        size: dvec2(300.0, 32.0),
    };
    let (bar, line, _) = look_parts(r, &look("Box", 5)).expect("box");
    assert_eq!((bar.pos.y + bar.size.y, bar.size.y), (72.0, 11.0));
    assert_eq!(bar.size.x, 300.0);
    assert!((line.size.x - 147.0).abs() < 0.01, "{}", line.size.x);
}

/// `Round`: the app's `SliderRound`, a pill the whole height whose fill grows
/// with the value and never shrinks below a circle.
#[test]
fn the_round_look_is_a_pill_filled_up_to_the_value() {
    let r = Rect {
        pos: dvec2(0.0, 0.0),
        size: dvec2(300.0, 32.0),
    };
    let (pill, empty, radius) = look_parts(r, &look("Round", 0)).expect("round");
    assert_eq!((pill.size.x, radius), (300.0, 15.5));
    assert_eq!(empty.size.x, empty.size.y, "at 0 the fill is a circle");
    let (_, full, _) = look_parts(r, &look("Round", 10)).expect("round");
    assert_eq!(full.size.x, 295.0);
}

/// A vertical bar keeps the line: the app's sliders only draw across.
#[test]
fn a_vertical_bar_ignores_the_look() {
    let r = Rect {
        pos: dvec2(0.0, 0.0),
        size: dvec2(24.0, 100.0),
    };
    let mut props = look("Round", 5);
    props.insert("orientation".to_string(), Value::Enum("Vertical".into()));
    assert!(look_parts(r, &props).is_none());
}
