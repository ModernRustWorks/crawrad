---
id: workspace-editorcolorscheme
kind: doc
target_file: src/workspace.rs
line_hint: 529
author: 
created: 
title: "EditorColorScheme"
tags: []
signature: "pub enum EditorColorScheme"
---

Presets for code editor color schemes.

# Examples

```rust
use crawrad_project::workspace::EditorColorScheme;

assert_eq!(EditorColorScheme::default(), EditorColorScheme::ModernDark);
```
