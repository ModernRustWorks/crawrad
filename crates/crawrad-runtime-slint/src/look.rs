//! Visual look, styling, and color helpers for Slint generated code.

/// Converts a hex color string (e.g. "#FF0000" or "#80FF0000") to Slint-compatible hex format `#rrggbbaa`.
pub fn hex_to_slint_color(hex: &str) -> String {
    let clean = hex.trim().trim_start_matches('#');
    match clean.len() {
        3 => {
            // #RGB -> #RRGGBBFF
            let mut s = String::with_capacity(9);
            s.push('#');
            for c in clean.chars() {
                s.push(c);
                s.push(c);
            }
            s.push_str("ff");
            s
        }
        6 => {
            // #RRGGBB -> #RRGGBBFF
            format!("#{clean}ff")
        }
        8 => {
            // #AARRGGBB (Delphi/CrawRAD) -> #RRGGBBAA (Slint/CSS)
            let a = &clean[0..2];
            let r = &clean[2..4];
            let g = &clean[4..6];
            let b = &clean[6..8];
            format!("#{r}{g}{b}{a}")
        }
        _ => "#000000ff".to_string(),
    }
}

/// Computes text alignment string for Slint markup.
pub fn text_alignment_slint(align: &str) -> &'static str {
    match align {
        "taRightJustify" => "right",
        "taCenter" => "center",
        _ => "left",
    }
}
