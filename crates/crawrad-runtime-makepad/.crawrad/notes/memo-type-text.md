---
id: memo-type-text
kind: doc
target_file: src/memo.rs
line_hint: 42
author: 
created: 
title: "type_text"
tags: []
signature: "pub fn type_text (cx : & mut Cx , input : & TextInputRef , typed : & str , max_length : i64 ,) -> Option < String >"
---

Inserts text at the current selection of `input` up to `max_length`.

Types `typed` at the input's selection, as the widget would for a key it
does not handle; the new text, or `None` when nothing changed.

# Examples

```rust,no_run
// Makepad Cx and TextInputRef require the Makepad application runtime.
use makepad_widgets::{Cx, TextInputRef};
use crawrad_runtime_makepad::memo::type_text;

fn insert(cx: &mut Cx, input: &TextInputRef) {
    let _ = type_text(cx, input, "a", 100);
}
```
