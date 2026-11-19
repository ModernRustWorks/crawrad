---
id: editor-footer-modified-label
kind: doc
target_file: src/editor_footer.rs
line_hint: 54
author: 
created: 
title: "modified_label"
tags: []
signature: "pub fn modified_label (modified : bool) -> & 'static str"
---

Returns the modified label when unsaved work is present, or empty string.

# Examples

```rust
use crawrad_host::editor_footer::modified_label;

assert_eq!(modified_label(true), "Modified");
assert_eq!(modified_label(false), "");
```
