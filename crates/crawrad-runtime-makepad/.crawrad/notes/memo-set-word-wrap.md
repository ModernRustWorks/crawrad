---
id: memo-set-word-wrap
kind: doc
target_file: src/memo.rs
line_hint: 69
author: 
created: 
title: "set_word_wrap"
tags: []
signature: "pub fn set_word_wrap (cx : & mut Cx , input : & TextInputRef , word_wrap : bool)"
---

Configures word wrapping flow for a text input widget.

`WordWrap`: a long line breaks at the edge (`RightWrap`, the widget's own
default) or runs on (`Right`).

# Examples

```rust,no_run
// Makepad Cx and TextInputRef require the Makepad application runtime.
use makepad_widgets::{Cx, TextInputRef};
use crawrad_runtime_makepad::memo::set_word_wrap;

fn toggle_wrap(cx: &mut Cx, input: &TextInputRef) {
    set_word_wrap(cx, input, true);
}
```
