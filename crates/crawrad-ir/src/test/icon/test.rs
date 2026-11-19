use crate::icon::{IconError, IconStep, rasterize, validate};

#[test]
fn filled_rect_full_grid_coverage() {
    let steps = [
        IconStep::Rect {
            x: 0.0,
            y: 0.0,
            w: 16.0,
            h: 16.0,
        },
        IconStep::Fill,
    ];
    let coverage = rasterize(&steps, 0.625, 16);
    assert_eq!(coverage.len(), 256);
    // The edge row is soft, as in `Sdf2d`: its centre is half a pixel inside.
    for y in 1..15 {
        for x in 1..15 {
            let val = coverage[y * 16 + x];
            assert_eq!(val, 1.0, "pixel ({x},{y}) expected 1.0, got {val}");
        }
    }
    assert!(
        coverage[0] > 0.0 && coverage[0] < 1.0,
        "the corner is antialiased"
    );
}

#[test]
fn empty_steps_returns_all_zeros() {
    let coverage = rasterize(&[], 0.625, 16);
    assert_eq!(coverage.len(), 256);
    for (i, &val) in coverage.iter().enumerate() {
        assert_eq!(val, 0.0, "pixel {i} expected 0.0, got {val}");
    }
}

#[test]
fn triangle_rasterization_coverage() {
    let steps = [
        IconStep::MoveTo(4.0, 3.5),
        IconStep::LineTo(12.5, 8.0),
        IconStep::LineTo(4.0, 12.5),
        IconStep::ClosePath,
        IconStep::Fill,
    ];
    let coverage = rasterize(&steps, 0.625, 16);
    let p_7_7 = coverage[7 * 16 + 7];
    let p_0_0 = coverage[0];
    let p_15_0 = coverage[15];

    assert!(
        (p_7_7 - 1.0).abs() < 0.01,
        "pixel (7,7) should be ~1.0, got {p_7_7}"
    );
    assert_eq!(p_0_0, 0.0, "pixel (0,0) should be 0");
    assert_eq!(p_15_0, 0.0, "pixel (15,0) should be 0");
}

#[test]
fn stroke_segment_coverage() {
    let steps = [
        IconStep::MoveTo(0.0, 8.0),
        IconStep::LineTo(16.0, 8.0),
        IconStep::Stroke,
    ];
    let coverage = rasterize(&steps, 0.625, 16);
    let row7 = coverage[7 * 16 + 8];
    let row8 = coverage[8 * 16 + 8];
    let row2 = coverage[2 * 16 + 8];

    assert_eq!(row7, row8, "row 7 and 8 should be symmetric");
    assert_eq!(row2, 0.0, "row 2 should be 0.0");
    assert!(
        row7 > 0.05,
        "row 7 coverage expected positive coverage, got {row7}"
    );
}

#[test]
fn all_values_within_range() {
    let steps = [
        IconStep::Circle {
            x: 8.0,
            y: 8.0,
            r: 4.0,
        },
        IconStep::Fill,
        IconStep::Box {
            x: 2.0,
            y: 2.0,
            w: 12.0,
            h: 12.0,
            r: 1.0,
        },
        IconStep::Stroke,
    ];
    let coverage = rasterize(&steps, 0.625, 16);
    for &val in &coverage {
        assert!((0.0..=1.0).contains(&val), "value {val} out of range 0..=1");
    }
}

/// Oversampling changes where the samples are, never the antialias: the
/// sample of each 4×4 block nearest a pixel centre agrees with the plain
/// rasterization within a quarter-pixel of ramp.
#[test]
fn oversampling_keeps_the_screen_antialias() {
    let steps = [
        IconStep::MoveTo(0.0, 8.0),
        IconStep::LineTo(16.0, 8.0),
        IconStep::Stroke,
    ];
    let plain = rasterize(&steps, 0.625, 16);
    let fine = crate::icon::rasterize_oversampled(&steps, 0.625, 16, 4);
    assert_eq!(fine.len(), 64 * 64);
    let at = |x: usize, y: usize| fine[y * 64 + x];
    // Rows 7 and 8 straddle the line; their plain coverage is the mean of the
    // fine samples across the pixel.
    for y in [7usize, 8] {
        let mean: f32 = (0..4).map(|k| at(32, y * 4 + k)).sum::<f32>() / 4.0;
        assert!(
            (mean - plain[y * 16 + 8]).abs() < 0.2,
            "row {y}: {mean} vs {}",
            plain[y * 16 + 8]
        );
    }
    assert_eq!(at(32, 2 * 4), 0.0, "far from the line nothing is painted");
}

#[test]
fn validate_empty_fails() {
    assert_eq!(validate(&[]), Err(IconError::Empty));
}

#[test]
fn validate_nothing_painted_fails() {
    let steps = [IconStep::MoveTo(1.0, 1.0)];
    assert_eq!(validate(&steps), Err(IconError::NothingPainted));

    let shape_after_paint = [
        IconStep::MoveTo(1.0, 1.0),
        IconStep::Fill,
        IconStep::MoveTo(2.0, 2.0),
    ];
    assert_eq!(validate(&shape_after_paint), Err(IconError::NothingPainted));
}

#[test]
fn validate_line_without_start_fails() {
    let line = [IconStep::LineTo(4.0, 4.0), IconStep::Stroke];
    assert_eq!(validate(&line), Err(IconError::LineWithoutStart(0)));

    let close = [IconStep::ClosePath, IconStep::Fill];
    assert_eq!(validate(&close), Err(IconError::LineWithoutStart(0)));

    let after_rect = [
        IconStep::Rect {
            x: 0.0,
            y: 0.0,
            w: 4.0,
            h: 4.0,
        },
        IconStep::LineTo(5.0, 5.0),
        IconStep::Stroke,
    ];
    assert_eq!(validate(&after_rect), Err(IconError::LineWithoutStart(1)));
}

#[test]
fn validate_not_finite_fails() {
    let nan_coord = [IconStep::MoveTo(f32::NAN, 1.0), IconStep::Stroke];
    assert_eq!(validate(&nan_coord), Err(IconError::NotFinite(0)));

    let inf_dim = [
        IconStep::Rect {
            x: 0.0,
            y: 0.0,
            w: f32::INFINITY,
            h: 4.0,
        },
        IconStep::Fill,
    ];
    assert_eq!(validate(&inf_dim), Err(IconError::NotFinite(0)));
}

#[test]
fn validate_out_of_grid_fails() {
    let neg_coord = [IconStep::MoveTo(-0.1, 1.0), IconStep::Stroke];
    assert_eq!(validate(&neg_coord), Err(IconError::OutOfGrid(0)));

    let beyond_grid = [IconStep::MoveTo(16.5, 1.0), IconStep::Stroke];
    assert_eq!(validate(&beyond_grid), Err(IconError::OutOfGrid(0)));

    let neg_radius = [
        IconStep::Circle {
            x: 8.0,
            y: 8.0,
            r: -1.0,
        },
        IconStep::Fill,
    ];
    assert_eq!(validate(&neg_radius), Err(IconError::OutOfGrid(0)));

    let rect_leaves_grid = [
        IconStep::Rect {
            x: 10.0,
            y: 10.0,
            w: 8.0,
            h: 4.0,
        },
        IconStep::Fill,
    ];
    assert_eq!(validate(&rect_leaves_grid), Err(IconError::OutOfGrid(0)));

    let box_leaves_grid = [
        IconStep::Box {
            x: 2.0,
            y: 10.0,
            w: 4.0,
            h: 10.0,
            r: 1.0,
        },
        IconStep::Fill,
    ];
    assert_eq!(validate(&box_leaves_grid), Err(IconError::OutOfGrid(0)));

    let neg_dim = [
        IconStep::Rect {
            x: 1.0,
            y: 1.0,
            w: -2.0,
            h: 4.0,
        },
        IconStep::Fill,
    ];
    assert_eq!(validate(&neg_dim), Err(IconError::OutOfGrid(0)));
}

#[test]
fn validate_native_like_valid_passes() {
    let triangle = [
        IconStep::MoveTo(4.0, 3.5),
        IconStep::LineTo(12.5, 8.0),
        IconStep::LineTo(4.0, 12.5),
        IconStep::ClosePath,
        IconStep::Fill,
    ];
    assert!(validate(&triangle).is_ok());

    let rect = [
        IconStep::Rect {
            x: 0.0,
            y: 0.0,
            w: 16.0,
            h: 16.0,
        },
        IconStep::Fill,
    ];
    assert!(validate(&rect).is_ok());

    let circle_and_box = [
        IconStep::Circle {
            x: 8.0,
            y: 8.0,
            r: 4.0,
        },
        IconStep::Fill,
        IconStep::Box {
            x: 2.0,
            y: 2.0,
            w: 12.0,
            h: 12.0,
            r: 1.0,
        },
        IconStep::Stroke,
    ];
    assert!(validate(&circle_and_box).is_ok());
}

#[test]
fn icon_error_display_and_error_impl() {
    use std::error::Error;

    let err = IconError::Empty;
    assert!(!err.to_string().is_empty());
    let _: &dyn Error = &err;

    let err = IconError::NothingPainted;
    assert!(!err.to_string().is_empty());

    let err = IconError::LineWithoutStart(3);
    assert!(err.to_string().contains('3'));

    let err = IconError::OutOfGrid(1);
    assert!(err.to_string().contains('1'));

    let err = IconError::NotFinite(2);
    assert!(err.to_string().contains('2'));
}

/// The engine bolt of `portadb-crawrad`, exactly as its `#[icon]` declares it.
fn engine_bolt() -> Vec<IconStep> {
    use IconStep::*;
    vec![
        MoveTo(10.0, 1.0),
        LineTo(10.0, 8.0),
        LineTo(3.5, 9.5),
        ClosePath,
        Fill,
        MoveTo(6.0, 15.0),
        LineTo(6.0, 8.0),
        LineTo(12.5, 6.5),
        ClosePath,
        Fill,
    ]
}

#[test]
fn the_engine_bolt_fills_both_halves() {
    let coverage = rasterize(&engine_bolt(), 0.625, 16);
    let top: f32 = coverage[..16 * 8].iter().sum();
    let bottom: f32 = coverage[16 * 8..].iter().sum();
    assert!(top > 10.0, "upper half of the bolt is empty: {top}");
    assert!(bottom > 10.0, "lower half of the bolt is empty: {bottom}");
}

#[test]
fn a_filled_path_wound_the_other_way_paints_nothing() {
    // The fill is the Sdf2d port: a path fills only when convex and wound
    // clockwise on screen. This pins the trap that left the first bolt blank.
    use IconStep::*;
    let ccw = [
        MoveTo(10.0, 1.0),
        LineTo(3.5, 9.5),
        LineTo(10.0, 8.0),
        ClosePath,
        Fill,
    ];
    assert_eq!(rasterize(&ccw, 0.625, 16).iter().sum::<f32>(), 0.0);
}
