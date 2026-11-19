---
id: rename-rename-preview
kind: doc
target_file: src/rename.rs
line_hint: 190
author: 
created: 
title: "Rename::preview"
tags: []
signature: "pub fn preview (& self) -> String"
---

The text of the preview: each file and its occurrences.

# Examples

```rust
use crawrad_host::rename::Rename;

let rename = Rename::default();
assert!(rename.preview().is_empty());
```
