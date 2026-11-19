use super::*;

const DESIGN: (f64, f64, f64, f64) = (20.0, 30.0, 100.0, 24.0);

fn a(left: bool, top: bool, right: bool, bottom: bool) -> Anchors {
    Anchors {
        left,
        top,
        right,
        bottom,
    }
}

#[test]
fn the_caption_bar_comes_out_of_the_window_before_the_form_grows() {
    let design = dvec2(640.0, 420.0);
    assert_eq!(grown(dvec2(640.0, 420.0), false, design), dvec2(0.0, 0.0));
    assert_eq!(
        grown(dvec2(700.0, 447.0), true, design),
        dvec2(60.0, 0.0),
        "27 of the 447 are Makepad's own caption bar"
    );
}

#[test]
fn the_default_anchors_keep_the_control_where_it_was_designed() {
    let g = dvec2(60.0, 40.0);
    assert_eq!(rect(DESIGN, Anchors::default(), g), DESIGN);
}

#[test]
fn anchored_on_both_sides_it_stretches_and_on_the_far_side_it_follows() {
    let g = dvec2(60.0, 40.0);
    assert_eq!(
        rect(DESIGN, a(true, true, true, true), g),
        (20.0, 30.0, 160.0, 64.0)
    );
    assert_eq!(
        rect(DESIGN, a(false, false, true, true), g),
        (80.0, 70.0, 100.0, 24.0)
    );
}

#[test]
fn a_shrunk_window_floors_the_painted_size_at_one() {
    let g = dvec2(-500.0, -500.0);
    let (_, _, w, h) = rect(DESIGN, a(true, true, true, true), g);
    assert_eq!((w, h), (1.0, 1.0));
}
