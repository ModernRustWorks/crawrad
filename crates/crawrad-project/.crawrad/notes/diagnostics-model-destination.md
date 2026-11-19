---
id: diagnostics-model-destination
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 134
author: 
created: 
title: "Destination"
tags: []
signature: "pub enum Destination"
---

Where clicking a diagnostic leads.

# Examples

```rust
use crawrad_project::diagnostics_model::Destination;

let dest = Destination::Unit { line: 1, column: 1 };
match dest {
    Destination::Unit { line, .. } => assert_eq!(line, 1),
    _ => unreachable!(),
}
```
