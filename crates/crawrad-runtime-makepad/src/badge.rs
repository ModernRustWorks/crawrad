//! **A Badge's pill drawing and kind colours, at run time**.

use crawrad_note::note;
use makepad_widgets::*;

#[note("badge-badge-text")]
pub fn badge_text(caption: &str, count: i64, max_count: i64) -> Option<String> {
    if count > 0 {
        if count > max_count {
            Some(format!("{max_count}+"))
        } else {
            Some(count.to_string())
        }
    } else if caption.is_empty() {
        None
    } else {
        Some(caption.to_string())
    }
}

#[note("badge-kind-colors")]
fn kind_colors(kind: &str) -> ([f32; 4], [f32; 4]) {
    match kind {
        "bkDefault" => ([0.22, 0.22, 0.28, 1.0], [0.95, 0.95, 0.98, 1.0]),
        "bkPrimary" => ([0.23, 0.35, 0.55, 1.0], [0.95, 0.95, 0.98, 1.0]),
        "bkInfo" => ([0.18, 0.50, 0.72, 1.0], [0.95, 0.95, 0.98, 1.0]),
        "bkWarning" => ([0.82, 0.55, 0.15, 1.0], [0.10, 0.10, 0.15, 1.0]),
        "bkDanger" => ([0.80, 0.22, 0.22, 1.0], [0.95, 0.95, 0.98, 1.0]),
        _ => ([0.23, 0.35, 0.55, 1.0], [0.95, 0.95, 0.98, 1.0]),
    }
}

#[note("badge-apply")]
pub fn apply(cx: &mut Cx, badge: &WidgetRef, kind: &str, outline: bool, enabled: bool) {
    let (bg, fg) = kind_colors(kind);
    let alpha = if enabled { 1.0 } else { 0.5 };

    if outline {
        let stroke_col = vec4(bg[0], bg[1], bg[2], bg[3] * alpha);
        let fill_col = vec4(0.0, 0.0, 0.0, 0.0);
        let text_col = vec4(bg[0], bg[1], bg[2], bg[3] * alpha);

        badge.apply_over(
            cx,
            live! {
                draw_bg: {
                    color: (fill_col),
                    border_color: (stroke_col),
                }
            },
        );
        badge.label(id!(label)).apply_over(
            cx,
            live! {
                draw_text: {
                    color: (text_col),
                }
            },
        );
    } else {
        let fill_col = vec4(bg[0], bg[1], bg[2], bg[3] * alpha);
        // `border_size` is a shader uniform (fixed at 1 in the DSL): a filled
        // badge hides its border by painting it the fill colour.
        let border_col = fill_col;
        let text_col = vec4(fg[0], fg[1], fg[2], fg[3] * alpha);

        badge.apply_over(
            cx,
            live! {
                draw_bg: {
                    color: (fill_col),
                    border_color: (border_col),
                }
            },
        );
        badge.label(id!(label)).apply_over(
            cx,
            live! {
                draw_text: {
                    color: (text_col),
                }
            },
        );
    }
}
