---
id: edit-set-alignment
kind: doc
target_file: src/edit.rs
line_hint: 16
author: 
created: 
title: "set_alignment"
tags: []
signature: "pub fn set_alignment (cx : & mut Cx , input : & TextInputRef , alignment : & str)"
---

Applies horizontal text alignment to a Makepad `TextInputRef`.

# Examples

```rust,no_run
// Makepad Cx and TextInputRef require the Makepad application runtime.
use makepad_widgets::{Cx, TextInputRef};
use crawrad_runtime_makepad::edit::set_alignment;

fn example(cx: &mut Cx, input: &TextInputRef) {
    set_alignment(cx, input, "taCenter");
}
```
