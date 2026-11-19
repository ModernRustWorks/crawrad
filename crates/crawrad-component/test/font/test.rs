use crate::pieces::font::Font;

#[test]
fn default_font_is_segoe_ui_9pt() {
    let f = Font::default();
    assert_eq!(f.name, "Segoe UI");
    assert_eq!(f.size, 9.0);
    assert!(!f.bold);
    assert!(!f.italic);
    assert!(!f.underline);
    assert!(!f.strike_out);
}

#[test]
fn fluent_constructors_modify_font() {
    let f = Font::new("Arial", 12.0)
        .bold(true)
        .italic(true)
        .underline(true)
        .strike_out(true);

    assert_eq!(f.name, "Arial");
    assert_eq!(f.size, 12.0);
    assert!(f.bold);
    assert!(f.italic);
    assert!(f.underline);
    assert!(f.strike_out);
}

#[test]
fn parse_and_display_font() {
    let f = Font::new("Tahoma", 10.0).bold(true);
    let s = f.to_string_formatted();
    assert_eq!(s, "Tahoma, 10.0pt, Bold");

    let parsed = Font::from_str_formatted(&s).expect("parsed successfully");
    assert_eq!(parsed.name, "Tahoma");
    assert_eq!(parsed.size, 10.0);
    assert!(parsed.bold);
    assert!(!parsed.italic);
}

/// The text colour, as bytes, round-trips through the descriptor.
#[test]
fn font_color_round_trips() {
    let f = Font::new("Tahoma", 10.0).with_color([0x3A, 0x5A, 0x8A, 255]);
    let s = f.to_string_formatted();
    assert_eq!(s, "Tahoma, 10.0pt, #3A5A8A");
    assert_eq!(Font::from_str_formatted(&s), Some(f));
    let half = Font::new("Tahoma", 10.0).with_color([1, 2, 3, 128]);
    assert_eq!(
        Font::from_str_formatted(&half.to_string_formatted()),
        Some(half)
    );
    assert_eq!(Font::default().color, [0, 0, 0, 0], "the theme's colour");
}
