---
id: cursor-show
kind: doc
target_file: src/cursor.rs
line_hint: 30
author: 
created: 
title: "show"
tags: []
signature: "pub fn show (cx : & mut Cx , name : & str)"
---

Shows the pointer named `name`, when it is one of ours.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::cursor::show;

fn example(cx: &mut Cx) {
    show(cx, "hand");
}
```
