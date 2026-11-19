---
id: workspace-editorcolorscheme-preset-labels
kind: doc
target_file: src/workspace.rs
line_hint: 557
author: 
created: 
title: "EditorColorScheme::preset_labels"
tags: []
signature: "pub fn preset_labels (base : EditorColorScheme) -> Vec < String >"
---

What the Color Scheme page's preset drop down shows, in [`Self::ALL`] order.

# Examples

```rust
use crawrad_project::workspace::EditorColorScheme;

let labels = EditorColorScheme::preset_labels(EditorColorScheme::ModernDark);
assert_eq!(labels.len(), 4);
```
