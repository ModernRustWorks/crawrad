//! Unit tests for canvas picture atlas and fit helpers.

use super::*;
use makepad_widgets::{Rect, dvec2};

#[test]
fn downscale_size_preserves_smaller_images() {
    assert_eq!(downscale_size(100, 80, 512), (100, 80));
    assert_eq!(downscale_size(512, 512, 512), (512, 512));
    assert_eq!(downscale_size(0, 50, 512), (0, 0));
}

#[test]
fn downscale_size_limits_longest_side_keeping_aspect() {
    // Width dominant: 1024x512 with max 512 -> 512x256
    assert_eq!(downscale_size(1024, 512, 512), (512, 256));
    // Height dominant: 600x1200 with max 512 -> 256x512
    assert_eq!(downscale_size(600, 1200, 512), (256, 512));
    // Square over max: 1000x1000 with max 512 -> 512x512
    assert_eq!(downscale_size(1000, 1000, 512), (512, 512));
}

#[test]
fn fit_rect_proportional_and_center() {
    let ctrl = Rect {
        pos: dvec2(10.0, 20.0),
        size: dvec2(100.0, 100.0),
    };
    // 200x100 in 100x100 -> fits as 100x50, centered vertically at y = 20 + 25 = 45
    let r = fit_rect(ctrl, 200.0, 100.0, false, true, true);
    assert_eq!(r.pos, dvec2(10.0, 45.0));
    assert_eq!(r.size, dvec2(100.0, 50.0));

    // 100x200 in 100x100 -> fits as 50x100, centered horizontally at x = 10 + 25 = 35
    let r2 = fit_rect(ctrl, 100.0, 200.0, false, true, true);
    assert_eq!(r2.pos, dvec2(35.0, 20.0));
    assert_eq!(r2.size, dvec2(50.0, 100.0));
}

#[test]
fn fit_rect_stretch_only() {
    let ctrl = Rect {
        pos: dvec2(10.0, 20.0),
        size: dvec2(100.0, 80.0),
    };
    // Stretch without proportional fills the control completely
    let r = fit_rect(ctrl, 30.0, 40.0, true, false, false);
    assert_eq!(r.pos, dvec2(10.0, 20.0));
    assert_eq!(r.size, dvec2(100.0, 80.0));
}

#[test]
fn fit_rect_neither_stretch_nor_proportional() {
    let ctrl = Rect {
        pos: dvec2(10.0, 20.0),
        size: dvec2(100.0, 100.0),
    };
    // Uses the image's own pixel size
    let r = fit_rect(ctrl, 40.0, 30.0, false, false, false);
    assert_eq!(r.pos, dvec2(10.0, 20.0));
    assert_eq!(r.size, dvec2(40.0, 30.0));

    // Centered uses image's own size centered in control
    let r_center = fit_rect(ctrl, 40.0, 30.0, false, false, true);
    assert_eq!(r_center.pos, dvec2(10.0 + 30.0, 20.0 + 35.0));
    assert_eq!(r_center.size, dvec2(40.0, 30.0));
}

#[test]
fn packing_two_pictures_do_not_overlap() {
    let mut atlas = PictureAtlas::new();
    let img1 = vec![0xFF0000FFu32; 100 * 100];
    let img2 = vec![0x00FF00FFu32; 80 * 80];

    let uv1 = atlas.insert_image(1, 100, 100, &img1);
    let uv2 = atlas.insert_image(2, 80, 80, &img2);

    let (x1, y1, w1, h1) = (uv1[0], uv1[1], uv1[2], uv1[3]);
    let (x2, y2, w2, h2) = (uv2[0], uv2[1], uv2[2], uv2[3]);

    let overlap = !(x1 + w1 <= x2 || x2 + w2 <= x1 || y1 + h1 <= y2 || y2 + h2 <= y1);
    assert!(
        !overlap,
        "packed pictures must not overlap: {uv1:?} vs {uv2:?}"
    );
}

#[test]
fn packing_too_big_set_resets_atlas() {
    let mut atlas = PictureAtlas::new();
    let img = vec![0xFFFFFFFFu32; 512 * 512];

    // Each shelf fits 4 images of 512 in 2048 width. 4 shelves fit 16 images.
    for i in 0..16 {
        atlas.insert_image(i, 512, 512, &img);
    }
    assert_eq!(atlas.map.len(), 16);

    // 17th image exceeds atlas height (4 * 512 = 2048), triggering atlas clear & repack
    let uv17 = atlas.insert_image(17, 512, 512, &img);
    assert_eq!(atlas.map.len(), 1, "atlas must reset on overflow");
    assert_eq!(uv17[0], 0.0, "new image starts at x=0 after reset");
    assert_eq!(uv17[1], 0.0, "new image starts at y=0 after reset");
}

#[test]
fn box_filter_averages_correctly() {
    // 2x2 source with 4 different colors
    let src = vec![0xFF000000, 0x00FF0000, 0x0000FF00, 0x000000FF];
    let dst = downscale_box_filter(&src, 2, 2, 1, 1);
    assert_eq!(dst.len(), 1);
    // Each component is averaged: 255 / 4 = 63 (0x3F)
    assert_eq!(dst[0], 0x3F3F3F3F);
}
