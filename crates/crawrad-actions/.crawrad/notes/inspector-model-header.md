---
id: inspector-model-header
kind: doc
target_file: src/inspector_model.rs
line_hint: 289
author: 
created: 
title: "header"
tags: []
signature: "pub fn header (doc : Option < & Document > , selection : & [NodeId]) -> String"
---

What the header of the panel says about the selection.

One selected: `name  ·  Class`, with the NAME and not the id (ADR 0008).
Several: the count, as in egui (`inspector.rs:58`). None: an em dash, which
is the rule of the shell — a number or a name that cannot be computed is
not invented.

# Examples

```rust
use crawrad_actions::inspector_model::header;

assert_eq!(header(None, &[]), "—");
```
