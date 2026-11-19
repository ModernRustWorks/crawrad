---
id: workspace-editordiagnosticcolors-rgba
kind: doc
target_file: src/workspace.rs
line_hint: 840
author: 
created: 
title: "EditorDiagnosticColors::rgba"
tags: []
signature: "pub fn rgba (& self) -> [[f32 ; 4] ; 6]"
---

Returns the six diagnostic colours as RGBA, in [`Self::fields`] order.

The six colours as RGBA, in [`Self::fields`] order; one that does not
parse falls back to its default.

# Examples

```rust
use crawrad_project::workspace::EditorDiagnosticColors;

let d = EditorDiagnosticColors::default();
assert_eq!(d.rgba().len(), 6);
```
