//! **The run time half of the `Edit`**: text alignment and helpers.

use crawrad_note::note;
use makepad_widgets::makepad_draw::text::selection::Cursor;
use makepad_widgets::*;

#[note("edit-align-x-of")]
pub fn align_x_of(alignment: &str) -> f64 {
    match alignment {
        "taCenter" => 0.5,
        "taRightJustify" => 1.0,
        _ => 0.0,
    }
}

#[note("edit-set-alignment")]
pub fn set_alignment(cx: &mut Cx, input: &TextInputRef, alignment: &str) {
    let ax = align_x_of(alignment);
    input.apply_over(
        cx,
        live! {
            label_align: { x: (ax) }
        },
    );
}

#[note("edit-set-numbers-only")]
pub fn set_numbers_only(cx: &mut Cx, input: &TextInputRef, numbers_only: bool) {
    input.apply_over(
        cx,
        live! {
            is_numeric_only: (numbers_only)
        },
    );
}

#[note("edit-set-text-hint")]
pub fn set_text_hint(cx: &mut Cx, input: &TextInputRef, text_hint: &str) {
    input.apply_over(
        cx,
        live! {
            empty_text: (text_hint)
        },
    );
}

#[note("edit-hold-case")]
pub fn hold_case(cx: &mut Cx, input: &TextInputRef, text: &str, char_case: &str) -> Option<String> {
    let turned = crawrad_ir::char_case::char_cased(text, char_case);
    if turned == text {
        return None;
    }
    let index = if turned.len() == text.len() {
        input.cursor().index
    } else {
        turned.len()
    };
    input.set_text(cx, &turned);
    put_caret(cx, input, index);
    Some(turned)
}

#[note("edit-hold-mask")]
pub fn hold_mask(
    cx: &mut Cx,
    input: &TextInputRef,
    before: &str,
    text: &str,
    edit_mask: &str,
) -> Option<String> {
    let mask = crawrad_runtime::mask::EditMask::parse(edit_mask);
    if mask.is_empty() {
        return None;
    }
    let caret = input.cursor().index.min(text.len());
    let typed_at = text
        .get(..caret)
        .map_or(text.chars().count(), |b| b.chars().count());
    let (shaped, next_caret_char) = mask.hold_edit(before, text, typed_at);
    let next_caret = shaped
        .char_indices()
        .nth(next_caret_char)
        .map_or(shaped.len(), |(byte, _)| byte);
    if shaped != text {
        input.set_text(cx, &shaped);
    }
    put_caret(cx, input, next_caret);
    Some(shaped)
}

#[note("edit-put-caret")]
pub fn put_caret(cx: &mut Cx, input: &TextInputRef, index: usize) {
    input.set_cursor(
        cx,
        Cursor {
            index,
            prefer_next_row: false,
        },
        false,
    );
}

#[note("edit-step-date-time-input")]
pub fn step_date_time_input(
    cx: &mut Cx,
    input: &TextInputRef,
    event: &Event,
    kind: &str,
    date_time: &mut String,
    min_date: &str,
    max_date: &str,
) -> bool {
    let Event::KeyDown(key) = event else {
        return false;
    };
    if key.key_code != KeyCode::ArrowUp && key.key_code != KeyCode::ArrowDown {
        return false;
    }
    if !input.key_focus(cx) {
        return false;
    }
    let up = key.key_code == KeyCode::ArrowUp;
    let kind = crawrad_runtime::date_time::DateTimeKind::parse_kind(kind);
    let text = input.text();
    let caret_byte = input.cursor().index.min(text.len());
    let caret_char = text
        .get(..caret_byte)
        .map_or(text.chars().count(), |b| b.chars().count());

    if let Some(iso) = crawrad_runtime::date_time::shown_to_iso(kind, &text)
        && let Some(stepped) =
            crawrad_runtime::date_time::step_iso(kind, &iso, caret_char, up, min_date, max_date)
    {
        *date_time = stepped;
        let shown = crawrad_runtime::date_time::iso_to_shown(kind, date_time);
        input.set_text(cx, &shown);
        let next_caret = shown
            .char_indices()
            .nth(caret_char)
            .map_or(shown.len(), |(byte, _)| byte);
        put_caret(cx, input, next_caret);
        return true;
    }
    false
}

#[cfg(test)]
#[path = "test/edit/test.rs"]
mod tests;
