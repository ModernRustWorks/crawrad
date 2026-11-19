---
id: targets-makepad-layout-is-inside-scroll-box
kind: doc
target_file: src/targets/makepad/layout.rs
line_hint: 127
author: 
created: 
title: "is_inside_scroll_box"
tags: []
signature: "pub fn is_inside_scroll_box (doc : & Document , id : & str) -> bool"
---

Returns true if the node is inside a `ScrollBox` container.

# Examples

```rust
use crawrad_codegen::targets::makepad::layout::is_inside_scroll_box;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert!(!is_inside_scroll_box(&doc, "btn"));
```
