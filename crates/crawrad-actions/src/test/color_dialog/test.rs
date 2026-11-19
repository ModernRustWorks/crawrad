use super::*;

#[test]
fn hue_survives_a_grey() {
    let initial = Color::rgb(0, 255, 255); // Hue 180 (cyan)
    let mut dlg = ColorDialog::new("Pick Color", initial, Vec::new());
    assert!((dlg.hue() - 180.0).abs() < 0.01);

    // Pick a grey patch (grey has no hue inherently)
    dlg.pick(Color::rgb(128, 128, 128));
    assert_eq!(dlg.chosen, Color::rgb(128, 128, 128));
    assert!(
        (dlg.hue() - 180.0).abs() < 0.01,
        "hue must survive a grey pick"
    );

    // Pick black (also grey)
    dlg.pick(Color::rgb(0, 0, 0));
    assert_eq!(dlg.chosen, Color::rgb(0, 0, 0));
    assert!(
        (dlg.hue() - 180.0).abs() < 0.01,
        "hue must survive picking black"
    );

    // Pick white (also grey)
    dlg.pick(Color::rgb(255, 255, 255));
    assert_eq!(dlg.chosen, Color::rgb(255, 255, 255));
    assert!(
        (dlg.hue() - 180.0).abs() < 0.01,
        "hue must survive picking white"
    );

    // Pick non-grey updates hue
    dlg.pick(Color::rgb(0, 0, 255)); // Blue (240 deg)
    assert!((dlg.hue() - 240.0).abs() < 0.01);
}

#[test]
fn square_and_strip_picks() {
    let initial = Color::rgb(255, 0, 0);
    let mut dlg = ColorDialog::new("Pick Color", initial, Vec::new());

    // Pick down the hue strip: 0.5 -> 180 deg (Cyan)
    dlg.pick_hue(0.5);
    assert!((dlg.hue() - 180.0).abs() < 0.01);

    // Pick on the saturation-value square
    dlg.pick_square(1.0, 1.0);
    assert_eq!(dlg.chosen, Color::rgb(0, 255, 255));
    let (s, v) = dlg.saturation_value();
    assert!((s - 1.0).abs() < 0.01);
    assert!((v - 1.0).abs() < 0.01);
}

#[test]
fn hex_and_channel_errors_leave_chosen_unchanged() {
    let initial = Color::rgb(10, 20, 30);
    let mut dlg = ColorDialog::new("Test", initial, Vec::new());
    let original = dlg.chosen;

    // Invalid hex syntax
    let err = dlg.set_hex("not a color");
    assert_eq!(err, Err("Not a colour: use #RRGGBB".to_string()));
    assert_eq!(dlg.chosen, original);

    // Invalid hex length
    let err2 = dlg.set_hex("#12345");
    assert_eq!(err2, Err("Not a colour: use #RRGGBB".to_string()));
    assert_eq!(dlg.chosen, original);

    // Invalid channel value out of bounds
    let ch_err = dlg.set_channel(Channel::Red, "256");
    assert_eq!(ch_err, Err("Use a number from 0 to 255".to_string()));
    assert_eq!(dlg.chosen, original);

    // Invalid channel non-numeric text
    let ch_err2 = dlg.set_channel(Channel::Green, "abc");
    assert_eq!(ch_err2, Err("Use a number from 0 to 255".to_string()));
    assert_eq!(dlg.chosen, original);
}

#[test]
fn accept_dedupes_and_caps_at_eight() {
    let c1 = Color::rgb(1, 1, 1);
    let c2 = Color::rgb(2, 2, 2);
    let c3 = Color::rgb(3, 3, 3);
    let c4 = Color::rgb(4, 4, 4);
    let c5 = Color::rgb(5, 5, 5);
    let c6 = Color::rgb(6, 6, 6);
    let c7 = Color::rgb(7, 7, 7);
    let c8 = Color::rgb(8, 8, 8);
    let c9 = Color::rgb(9, 9, 9);

    // Given 9 recent colors, ColorDialog::new truncates to RECENT_MAX (8)
    let mut dlg = ColorDialog::new("Recent Test", c1, vec![c2, c3, c4, c5, c6, c7, c8, c9]);
    assert_eq!(dlg.recent.len(), 8);
    assert_eq!(dlg.recent[7], c9); // c2..=c9 is 8 items

    // Now chosen is c4 (which already exists in recent list)
    dlg.pick(c4);
    let (chosen, recent) = dlg.accept();
    assert_eq!(chosen, c4);
    assert_eq!(recent.len(), 8);
    assert_eq!(recent[0], c4, "chosen color moves to the front");
    assert_eq!(
        recent.iter().filter(|c| **c == c4).count(),
        1,
        "no duplicates"
    );

    // Test with brand new color
    let c_new = Color::rgb(99, 99, 99);
    let dlg2 = ColorDialog::new("New Pick", c_new, vec![c1, c2, c3]);
    let (chosen2, recent2) = dlg2.accept();
    assert_eq!(chosen2, c_new);
    assert_eq!(recent2, vec![c_new, c1, c2, c3]);
}

#[test]
fn current_never_changes() {
    let initial = Color::rgb(12, 34, 56);
    let mut dlg = ColorDialog::new("Immutable Current", initial, Vec::new());

    dlg.pick(Color::rgb(255, 0, 0));
    dlg.pick_hue(0.25);
    dlg.pick_square(0.8, 0.4);
    let _ = dlg.set_hex("#AABBCC");
    let _ = dlg.set_channel(Channel::Red, "200");

    assert_eq!(dlg.current, initial);
}

#[test]
fn fields_formatting_and_basic_palette() {
    let mut dlg = ColorDialog::new("Fields", Color::rgb(255, 128, 64), Vec::new());
    let f = dlg.fields();
    assert_eq!(f.hex, "#FF8040");
    assert_eq!(f.red, "255");
    assert_eq!(f.green, "128");
    assert_eq!(f.blue, "64");
    assert_eq!(f.alpha, "255");

    let _ = dlg.set_channel(Channel::Alpha, "100");
    let f2 = dlg.fields();
    assert_eq!(f2.hex, "#FF804064");
    assert_eq!(f2.alpha, "100");

    assert_eq!(ColorDialog::basic().len(), 16);
    assert_eq!(ColorDialog::basic()[0].0, "Black");
}
