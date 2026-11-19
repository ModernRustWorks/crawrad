---
id: workspace-editorcolorscheme-preset
kind: doc
target_file: src/workspace.rs
line_hint: 854
author: 
created: 
title: "EditorColorScheme::preset"
tags: []
signature: "pub fn preset (self) -> Option < EditorCustomColors >"
---

The token colors a preset brings into the custom fields when picked.

The token colors a preset brings into the custom fields when picked;
`None` for `Custom`, which keeps what the user typed.

# Examples

```rust
use crawrad_project::workspace::EditorColorScheme;

assert!(EditorColorScheme::ModernDark.preset().is_some());
assert!(EditorColorScheme::Custom.preset().is_none());
```
