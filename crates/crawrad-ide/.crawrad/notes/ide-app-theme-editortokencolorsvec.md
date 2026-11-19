---
id: ide-app-theme-editortokencolorsvec
kind: doc
target_file: src/ide_app/theme.rs
line_hint: 53
author: 
created: 
title: "EditorTokenColorsVec"
tags: []
signature: "pub struct EditorTokenColorsVec"
---

Token color Vec4 vectors ready for application in Makepad's CodeEditor.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let colors = EditorTokenColorsVec::from(EditorTokenColors::default());
assert!(colors.branch_keyword.x >= 0.0);
```
