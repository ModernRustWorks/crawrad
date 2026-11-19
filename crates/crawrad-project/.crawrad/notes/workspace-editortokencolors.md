---
id: workspace-editortokencolors
kind: doc
target_file: src/workspace.rs
line_hint: 882
author: 
created: 
title: "EditorTokenColors"
tags: []
signature: "pub struct EditorTokenColors"
---

Editor token colors as RGBA quartets [f32; 4].

# Examples

```rust
use crawrad_project::workspace::{EditorColorScheme, EditorCustomColors};

let tc = EditorColorScheme::ModernDark.colors(&EditorCustomColors::default());
assert_eq!(tc.bg, [0.118, 0.118, 0.118, 1.0]);
```
