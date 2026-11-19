use super::*;

#[test]
fn pure_colors_to_and_from_hsv() {
    let red = Color::rgb(255, 0, 0);
    assert_eq!(red.to_hsv(), (0.0, 1.0, 1.0));
    assert_eq!(Color::from_hsv(0.0, 1.0, 1.0, 255), red);

    let green = Color::rgb(0, 255, 0);
    assert_eq!(green.to_hsv(), (120.0, 1.0, 1.0));
    assert_eq!(Color::from_hsv(120.0, 1.0, 1.0, 255), green);

    let blue = Color::rgb(0, 0, 255);
    assert_eq!(blue.to_hsv(), (240.0, 1.0, 1.0));
    assert_eq!(Color::from_hsv(240.0, 1.0, 1.0, 255), blue);

    let white = Color::rgb(255, 255, 255);
    assert_eq!(white.to_hsv(), (0.0, 0.0, 1.0));
    assert_eq!(Color::from_hsv(0.0, 0.0, 1.0, 255), white);

    let black = Color::rgb(0, 0, 0);
    assert_eq!(black.to_hsv(), (0.0, 0.0, 0.0));
    assert_eq!(Color::from_hsv(0.0, 0.0, 0.0, 255), black);

    let grey = Color::rgb(128, 128, 128);
    assert_eq!(grey.to_hsv(), (0.0, 0.0, 128.0 / 255.0));
    assert_eq!(Color::from_hsv(0.0, 0.0, 128.0 / 255.0, 255), grey);
}

#[test]
fn hsv_round_trip_for_standard_and_sampled_colors() {
    for (_name, c) in Color::STANDARD {
        let (h, s, v) = c.to_hsv();
        assert_eq!(Color::from_hsv(h, s, v, c.a), c);
    }

    for hex in ["#FFA463", "#4EC9B0", "#2B579A"] {
        let c = Color::from_hex(hex).expect("valid hex color");
        let (h, s, v) = c.to_hsv();
        assert_eq!(
            Color::from_hsv(h, s, v, c.a),
            c,
            "round trip failed for {hex}"
        );
    }
}

#[test]
fn hsv_angle_wrap_and_clamps() {
    let red = Color::rgb(255, 0, 0);
    assert_eq!(Color::from_hsv(360.0, 1.0, 1.0, 255), red);
    assert_eq!(Color::from_hsv(720.0, 1.0, 1.0, 255), red);
    assert_eq!(Color::from_hsv(-360.0, 1.0, 1.0, 255), red);
    assert_eq!(Color::from_hsv(0.0, 2.0, 2.0, 255), red);
    assert_eq!(Color::from_hsv(0.0, -1.0, 0.0, 255), Color::rgb(0, 0, 0));
}
