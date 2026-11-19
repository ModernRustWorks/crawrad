---
id: edit-put-caret
kind: doc
target_file: src/edit.rs
line_hint: 91
author: 
created: 
title: "put_caret"
tags: []
signature: "pub fn put_caret (cx : & mut Cx , input : & TextInputRef , index : usize)"
---

Puts the input's caret on the byte `index`, with no selection.

# Examples

```rust,no_run
// Makepad Cx and TextInputRef require the Makepad application runtime.
use makepad_widgets::{Cx, TextInputRef};
use crawrad_runtime_makepad::edit::put_caret;

fn example(cx: &mut Cx, input: &TextInputRef) {
    put_caret(cx, input, 0);
}
```
