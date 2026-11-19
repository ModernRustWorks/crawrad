//! **The run time half of the `Memo`**: the keys a multiline text wants
//! and the most characters it takes.
//!
//! makepad 1.0.0's `TextInput` inserts a new line only on Shift+Enter — a
//! plain Enter is reported as `Returned` and changes nothing
//! (`text_input.rs:1533-1578`) — it ignores Tab, and it has no length limit.
//! A classic RAD `Memo` starts a new line on Enter (`WantReturns`), types a tab
//! when asked to (`WantTabs`) and stops at `MaxLength`. The generated `App`
//! calls these with what the form says at that moment.

use crawrad_note::note;
use makepad_widgets::*;

#[note("memo-clamp")]
pub fn clamp(text: &str, max_length: i64) -> &str {
    if max_length <= 0 {
        return text;
    }
    match text.char_indices().nth(max_length as usize) {
        Some((at, _)) => &text[..at],
        None => text,
    }
}

#[note("memo-typed-at")]
pub fn typed_at(
    text: &str,
    start: usize,
    end: usize,
    typed: &str,
    max_length: i64,
) -> Option<(String, usize)> {
    let (start, end) = (start.min(text.len()), end.min(text.len()));
    let kept = text[..start].chars().count() + text[end..].chars().count();
    if max_length > 0 && kept + typed.chars().count() > max_length as usize {
        return None;
    }
    let out = format!("{}{typed}{}", &text[..start], &text[end..]);
    Some((out, start + typed.len()))
}

#[note("memo-type-text")]
pub fn type_text(
    cx: &mut Cx,
    input: &TextInputRef,
    typed: &str,
    max_length: i64,
) -> Option<String> {
    let sel = input.selection();
    let text = input.text();
    let (out, at) = typed_at(&text, sel.start().index, sel.end().index, typed, max_length)?;
    input.set_text(cx, &out);
    crate::edit::put_caret(cx, input, at);
    Some(out)
}

#[note("memo-hold-to")]
pub fn hold_to(cx: &mut Cx, input: &TextInputRef, text: &str, max_length: i64) -> Option<String> {
    let kept = clamp(text, max_length);
    if kept.len() == text.len() {
        return None;
    }
    let kept = kept.to_string();
    input.set_text(cx, &kept);
    crate::edit::put_caret(cx, input, kept.len());
    Some(kept)
}

#[note("memo-set-word-wrap")]
pub fn set_word_wrap(cx: &mut Cx, input: &TextInputRef, word_wrap: bool) {
    if word_wrap {
        input.apply_over(cx, live! { flow: RightWrap });
    } else {
        input.apply_over(cx, live! { flow: Right });
    }
}

#[cfg(test)]
#[path = "test/memo/test.rs"]
mod tests;
