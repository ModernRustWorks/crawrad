---
id: editor-footer-language-label
kind: doc
target_file: src/editor_footer.rs
line_hint: 19
author: 
created: 
title: "language_label"
tags: []
signature: "pub fn language_label (path : Option < & Path >) -> & 'static str"
---

Detects the friendly language name from the active file path.

# Examples

```rust
use std::path::Path;
use crawrad_host::editor_footer::language_label;

assert_eq!(language_label(Some(Path::new("src/main.rs"))), "Rust");
```
