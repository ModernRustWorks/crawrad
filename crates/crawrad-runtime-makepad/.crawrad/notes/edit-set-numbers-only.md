---
id: edit-set-numbers-only
kind: doc
target_file: src/edit.rs
line_hint: 27
author: 
created: 
title: "set_numbers_only"
tags: []
signature: "pub fn set_numbers_only (cx : & mut Cx , input : & TextInputRef , numbers_only : bool)"
---

Sets whether a Makepad `TextInputRef` only accepts digits.

# Examples

```rust,no_run
// Makepad Cx and TextInputRef require the Makepad application runtime.
use makepad_widgets::{Cx, TextInputRef};
use crawrad_runtime_makepad::edit::set_numbers_only;

fn example(cx: &mut Cx, input: &TextInputRef) {
    set_numbers_only(cx, input, true);
}
```
