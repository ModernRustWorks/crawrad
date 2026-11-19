---
id: workspace-preferences-color-fields
kind: doc
target_file: src/workspace.rs
line_hint: 1048
author: 
created: 
title: "Preferences::color_fields"
tags: []
signature: "pub fn color_fields (& self) -> [& str ; 15]"
---

The fifteen colours of the Color Scheme page, in its input order.

The fifteen colours of the Color Scheme page, in its input order: the
nine syntax colours ([`EditorCustomColors::fields`]), then the six
diagnostic ones ([`EditorDiagnosticColors::fields`]).

# Examples

```rust
use crawrad_project::workspace::Preferences;

let p = Preferences::default();
assert_eq!(p.color_fields().len(), 15);
```
