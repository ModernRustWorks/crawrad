---
id: explorer-model-folder-label
kind: doc
target_file: src/explorer_model.rs
line_hint: 296
author: 
created: 
title: "folder_label"
tags: []
signature: "pub fn folder_label (folder : & Path) -> Option < String >"
---

The label of a plan folder, or `None` for the member root.

`None` because the member node, one line above, already names it: repeating the
name there wasted a line of a narrow panel to say what was just said, and suggested
they were two places. Before the tree the label was necessary — there was no
member node at all.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_model::folder_label;

assert_eq!(folder_label(Path::new("")), None);
assert_eq!(folder_label(Path::new("src")), Some("src/".to_string()));
```
