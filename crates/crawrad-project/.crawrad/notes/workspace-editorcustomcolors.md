---
id: workspace-editorcustomcolors
kind: doc
target_file: src/workspace.rs
line_hint: 685
author: 
created: 
title: "EditorCustomColors"
tags: []
signature: "pub struct EditorCustomColors"
---

Custom colors configured by user for editor syntax tokens.

# Examples

```rust
use crawrad_project::workspace::EditorCustomColors;

let c = EditorCustomColors::default();
assert_eq!(c.keyword, "#ff7b72");
```
