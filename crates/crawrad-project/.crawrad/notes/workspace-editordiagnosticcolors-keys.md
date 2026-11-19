---
id: workspace-editordiagnosticcolors-keys
kind: doc
target_file: src/workspace.rs
line_hint: 781
author: 
created: 
title: "EditorDiagnosticColors::KEYS"
tags: []
signature: "pub const KEYS : [& 'static str ; 6] = [\"editor-diagnostic-error\" , \"editor-diagnostic-warning\" , \"editor-diagnostic-info\" , \"editor-diagnostic-error-band\" , \"editor-diagnostic-warning-band\" , \"editor-diagnostic-info-band\" ,]"
---

The preference keys, in [`Self::fields`] order.

# Examples

```rust
use crawrad_project::workspace::EditorDiagnosticColors;

assert_eq!(EditorDiagnosticColors::KEYS.len(), 6);
```
