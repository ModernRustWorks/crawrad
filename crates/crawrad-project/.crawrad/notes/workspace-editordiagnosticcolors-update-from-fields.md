---
id: workspace-editordiagnosticcolors-update-from-fields
kind: doc
target_file: src/workspace.rs
line_hint: 814
author: 
created: 
title: "EditorDiagnosticColors::update_from_fields"
tags: []
signature: "pub fn update_from_fields (& mut self , inputs : & [String ; 6]) -> bool"
---

Updates the colours from the inputs, in [`Self::fields`] order.

Updates the colours from the inputs, in [`Self::fields`] order; an
empty input or one that is not a colour keeps its colour. Returns
whether any colour changed.

# Examples

```rust
use crawrad_project::workspace::EditorDiagnosticColors;

let mut d = EditorDiagnosticColors::default();
let inputs: [String; 6] = std::array::from_fn(|_| "#112233".to_string());
assert!(d.update_from_fields(&inputs));
assert_eq!(d.error, "#112233");
```
