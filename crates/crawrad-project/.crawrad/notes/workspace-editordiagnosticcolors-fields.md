---
id: workspace-editordiagnosticcolors-fields
kind: doc
target_file: src/workspace.rs
line_hint: 791
author: 
created: 
title: "EditorDiagnosticColors::fields"
tags: []
signature: "pub fn fields (& self) -> [& str ; 6]"
---

The six colours in the order the Options page lists their inputs.

The six colours in the order the Options page lists their inputs:
error, warning, info text, then their bands.

# Examples

```rust
use crawrad_project::workspace::EditorDiagnosticColors;

let d = EditorDiagnosticColors::default();
assert_eq!(d.fields().len(), 6);
```
