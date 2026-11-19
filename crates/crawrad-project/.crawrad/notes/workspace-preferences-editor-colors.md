---
id: workspace-preferences-editor-colors
kind: doc
target_file: src/workspace.rs
line_hint: 1043
author: 
created: 
title: "Preferences::editor_colors"
tags: []
signature: "pub fn editor_colors (& self) -> EditorTokenColors"
---

Gets active token colors for code editor.

# Examples

```rust
use crawrad_project::workspace::Preferences;

let p = Preferences::default();
assert_eq!(p.editor_colors().bg, [0.118, 0.118, 0.118, 1.0]);
```
