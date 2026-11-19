---
id: workspace-preferences-reset-colors
kind: doc
target_file: src/workspace.rs
line_hint: 1081
author: 
created: 
title: "Preferences::reset_colors"
tags: []
signature: "pub fn reset_colors (& mut self)"
---

Resets color scheme and diagnostic colors to their defaults.

Reset Default: Modern Dark with its colours, base back to Modern Dark,
and the default diagnostic colours.

# Examples

```rust
use crawrad_project::workspace::{EditorColorScheme, Preferences};

let mut p = Preferences::default();
p.editor_color_scheme = EditorColorScheme::Monokai;
p.reset_colors();
assert_eq!(p.editor_color_scheme, EditorColorScheme::ModernDark);
```
