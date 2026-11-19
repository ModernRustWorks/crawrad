---
id: workspace-editorcolorscheme-short-name
kind: doc
target_file: src/workspace.rs
line_hint: 547
author: 
created: 
title: "EditorColorScheme::short_name"
tags: []
signature: "pub fn short_name (self) -> & 'static str"
---

Short display name of the preset.

# Examples

```rust
use crawrad_project::workspace::EditorColorScheme;

assert_eq!(EditorColorScheme::ModernDark.short_name(), "Modern Dark");
```
