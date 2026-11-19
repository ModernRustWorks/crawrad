---
id: workspace-preferences-pick-scheme
kind: doc
target_file: src/workspace.rs
line_hint: 1070
author: 
created: 
title: "Preferences::pick_scheme"
tags: []
signature: "pub fn pick_scheme (& mut self , scheme : EditorColorScheme)"
---

Sets active scheme.

Sets active scheme. If not `Custom`, also updates `editor_color_base`
and copies preset colors into `editor_custom_colors`.

# Examples

```rust
use crawrad_project::workspace::{EditorColorScheme, Preferences};

let mut p = Preferences::default();
p.pick_scheme(EditorColorScheme::Monokai);
assert_eq!(p.editor_color_scheme, EditorColorScheme::Monokai);
```
