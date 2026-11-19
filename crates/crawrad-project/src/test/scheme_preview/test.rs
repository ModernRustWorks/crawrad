use super::*;
use crate::workspace::{EditorColorScheme, Preferences};

#[test]
fn preview_lines_structure_and_line_two_text() {
    let sample = lines();
    assert_eq!(sample.len(), 6);
    assert_eq!(sample[3].kind, LineKind::Error);
    assert_eq!(sample[3].number, 4);
    assert_eq!(sample[4].kind, LineKind::Warning);
    assert_eq!(sample[4].number, 5);

    let line2_text: String = sample[1].spans.iter().map(|(text, _)| *text).collect();
    assert_eq!(line2_text, "fn load(id: u32) -> Order {");
}

#[test]
fn custom_keyword_color_and_normal_band() {
    let mut prefs = Preferences {
        editor_color_scheme: EditorColorScheme::Custom,
        ..Preferences::default()
    };
    prefs.editor_custom_colors.keyword = "#ff0000".to_string();

    let kw_color = color(&prefs, Role::Keyword);
    assert_eq!(kw_color, [1.0, 0.0, 0.0, 1.0]);

    assert_eq!(band(&prefs, LineKind::Normal), None);
}

#[test]
fn diagnostic_bands_and_background_colors() {
    let prefs = Preferences::default();
    let err_band = band(&prefs, LineKind::Error);
    let warn_band = band(&prefs, LineKind::Warning);
    assert!(err_band.is_some());
    assert!(warn_band.is_some());

    let bg_color = background(&prefs);
    assert_eq!(bg_color, prefs.editor_colors().bg);
}

#[test]
fn painted_resolves_all_preview_rows() {
    let prefs = Preferences::default();
    let rows = painted(&prefs);
    assert_eq!(rows.len(), 6);

    // Line 1 is normal -> number color is GUTTER, band is None
    assert_eq!(rows[0].number, "1");
    assert_eq!(rows[0].number_color, GUTTER);
    assert_eq!(rows[0].band, None);
    assert_eq!(rows[0].spans[0].0, "// load the order and its lines");

    // Line 4 is error -> number color matches Role::Error, band is Some
    assert_eq!(rows[3].number, "4");
    assert_eq!(rows[3].number_color, color(&prefs, Role::Error));
    assert_eq!(rows[3].band, band(&prefs, LineKind::Error));

    // Line 5 is warning -> number color matches Role::Warning, band is Some
    assert_eq!(rows[4].number, "5");
    assert_eq!(rows[4].number_color, color(&prefs, Role::Warning));
    assert_eq!(rows[4].band, band(&prefs, LineKind::Warning));
}
