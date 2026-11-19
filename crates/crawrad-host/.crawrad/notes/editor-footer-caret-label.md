---
id: editor-footer-caret-label
kind: doc
target_file: src/editor_footer.rs
line_hint: 40
author: 
created: 
title: "caret_label"
tags: []
signature: "pub fn caret_label (line : usize , byte : usize , line_text : & str) -> String"
---

Formats the caret cell label: 1-based line number and 1-based visual column.

# Examples

```rust
use crawrad_host::editor_footer::caret_label;

assert_eq!(caret_label(0, 0, ""), "Ln 1, Col 1");
```
