---
id: inspector-model-window-for
kind: doc
target_file: src/inspector_model.rs
line_hint: 679
author: 
created: 
title: "window_for"
tags: []
signature: "pub fn window_for (rows : & [Row] , key : & str) -> Window"
---

Which window opens for the property `key` among the Inspector's `rows`.

It is the row's EDITOR that says, not a `match` on the key in the shell:
`editors::collection` says a collection opens the Items Editor,
`editors::lines` that a text opens the String List Editor, and the rest
(`editors::align`) opens a drawing over the form, with no text at all.

# Examples

```rust
use crawrad_actions::inspector_model::{window_for, Window};

let win = window_for(&[], "caption");
assert_eq!(win, Window::Pad);
```
