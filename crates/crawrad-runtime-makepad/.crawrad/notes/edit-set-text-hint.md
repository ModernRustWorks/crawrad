---
id: edit-set-text-hint
kind: doc
target_file: src/edit.rs
line_hint: 37
author: 
created: 
title: "set_text_hint"
tags: []
signature: "pub fn set_text_hint (cx : & mut Cx , input : & TextInputRef , text_hint : & str)"
---

Sets the placeholder text (`empty_text`) of a Makepad `TextInputRef`.

# Examples

```rust,no_run
// Makepad Cx and TextInputRef require the Makepad application runtime.
use makepad_widgets::{Cx, TextInputRef};
use crawrad_runtime_makepad::edit::set_text_hint;

fn example(cx: &mut Cx, input: &TextInputRef) {
    set_text_hint(cx, input, "Type here...");
}
```
