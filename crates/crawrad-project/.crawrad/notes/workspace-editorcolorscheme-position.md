---
id: workspace-editorcolorscheme-position
kind: doc
target_file: src/workspace.rs
line_hint: 567
author: 
created: 
title: "EditorColorScheme::position"
tags: []
signature: "pub fn position (self) -> usize"
---

Its place in [`Self::ALL`], the drop down's item.

# Examples

```rust
use crawrad_project::workspace::EditorColorScheme;

assert_eq!(EditorColorScheme::ModernDark.position(), 0);
```
