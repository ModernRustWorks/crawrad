---
id: memo-hold-to
kind: doc
target_file: src/memo.rs
line_hint: 57
author: 
created: 
title: "hold_to"
tags: []
signature: "pub fn hold_to (cx : & mut Cx , input : & TextInputRef , text : & str , max_length : i64) -> Option < String >"
---

Holds the input to `max_length`: the text cut, or `None` when it fits.

# Examples

```rust,no_run
// Makepad Cx and TextInputRef require the Makepad application runtime.
use makepad_widgets::{Cx, TextInputRef};
use crawrad_runtime_makepad::memo::hold_to;

fn enforce(cx: &mut Cx, input: &TextInputRef) {
    let _ = hold_to(cx, input, "12345", 3);
}
```
