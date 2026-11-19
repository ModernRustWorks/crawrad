---
id: editor-footer-mode-label
kind: doc
target_file: src/editor_footer.rs
line_hint: 49
author: 
created: 
title: "mode_label"
tags: []
signature: "pub fn mode_label (editable : bool) -> & 'static str"
---

Returns the mode label based on whether the active file is editable.

# Examples

```rust
use crawrad_host::editor_footer::mode_label;

assert_eq!(mode_label(true), "Insert");
assert_eq!(mode_label(false), "Read only");
```
