---
id: workspace-editordiagnosticcolors
kind: doc
target_file: src/workspace.rs
line_hint: 756
author: 
created: 
title: "EditorDiagnosticColors"
tags: []
signature: "pub struct EditorDiagnosticColors"
---

The colours of a line that carries a diagnostic.

The colours of a line that carries a diagnostic: its message and line
number (text) and the band behind it, end to end, per severity.

# Examples

```rust
use crawrad_project::workspace::EditorDiagnosticColors;

let d = EditorDiagnosticColors::default();
assert_eq!(d.error, "#f38ba8");
```
