//! Unit tests for window capture cropping and color format translation.

use crate::crop_bgra;

#[test]
fn crop_bgra_whole_image() {
    // 2x2 image, BGRA layout
    let src = vec![
        10, 20, 30, 40, // (0, 0): B=10, G=20, R=30
        50, 60, 70, 80, // (1, 0): B=50, G=60, R=70
        90, 100, 110, 120, // (0, 1): B=90, G=100, R=110
        130, 140, 150, 160, // (1, 1): B=130, G=140, R=150
    ];
    let captured = crop_bgra(&src, 2, 2, [0, 0, 2, 2]).expect("full crop should succeed");
    assert_eq!(captured.width, 2);
    assert_eq!(captured.height, 2);
    let expected_rgba = vec![
        30, 20, 10, 255, 70, 60, 50, 255, 110, 100, 90, 255, 150, 140, 130, 255,
    ];
    assert_eq!(captured.rgba, expected_rgba);
}

#[test]
fn crop_bgra_inner_crop_with_swap_on_known_pixel() {
    // 3x3 image, all black except pixel (1, 1) which has known color
    let mut src = vec![0u8; 3 * 3 * 4];
    let index = (3 + 1) * 4;
    src[index] = 11; // Blue
    src[index + 1] = 22; // Green
    src[index + 2] = 33; // Red
    src[index + 3] = 44; // Alpha (ignored, should become 255)

    let captured = crop_bgra(&src, 3, 3, [1, 1, 1, 1]).expect("inner crop should succeed");
    assert_eq!(captured.width, 1);
    assert_eq!(captured.height, 1);
    assert_eq!(captured.rgba, vec![33, 22, 11, 255]);
}

#[test]
fn crop_bgra_region_partly_outside_clamps() {
    // 4x4 image
    let mut src = vec![0u8; 4 * 4 * 4];
    for y in 0..4 {
        for x in 0..4 {
            let index = (y * 4 + x) * 4;
            src[index] = (x * 10) as u8;
            src[index + 1] = (y * 10) as u8;
            src[index + 2] = ((x + y) * 10) as u8;
            src[index + 3] = 0;
        }
    }

    // Top-left extending outside bounds: [-1, -1, 3, 3] -> clamped to [0, 0, 2, 2]
    let captured =
        crop_bgra(&src, 4, 4, [-1, -1, 3, 3]).expect("partly outside top-left should clamp");
    assert_eq!(captured.width, 2);
    assert_eq!(captured.height, 2);

    // Bottom-right extending outside bounds: [2, 2, 4, 4] -> clamped to [2, 2, 2, 2]
    let captured_br =
        crop_bgra(&src, 4, 4, [2, 2, 4, 4]).expect("partly outside bottom-right should clamp");
    assert_eq!(captured_br.width, 2);
    assert_eq!(captured_br.height, 2);
}

#[test]
fn crop_bgra_region_fully_outside_returns_none() {
    let src = vec![0u8; 4 * 4 * 4];

    // Fully to the left
    assert!(crop_bgra(&src, 4, 4, [-10, 0, 5, 2]).is_none());
    // Fully to the right
    assert!(crop_bgra(&src, 4, 4, [10, 0, 2, 2]).is_none());
    // Fully above
    assert!(crop_bgra(&src, 4, 4, [0, -10, 2, 5]).is_none());
    // Fully below
    assert!(crop_bgra(&src, 4, 4, [0, 10, 2, 2]).is_none());

    // Empty / non-positive dimensions
    assert!(crop_bgra(&src, 4, 4, [0, 0, 0, 2]).is_none());
    assert!(crop_bgra(&src, 4, 4, [0, 0, 2, -1]).is_none());
    assert!(crop_bgra(&src, 4, 4, [0, 0, -2, 2]).is_none());

    // Empty source image
    assert!(crop_bgra(&[], 0, 0, [0, 0, 1, 1]).is_none());
    // Buffer too small
    assert!(crop_bgra(&[0; 10], 4, 4, [0, 0, 1, 1]).is_none());
}
