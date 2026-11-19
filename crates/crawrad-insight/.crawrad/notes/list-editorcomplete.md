---
id: list-editorcomplete
kind: doc
target_file: src/list.rs
line_hint: 36
author: 
created: 
title: "EditorComplete"
tags: []
signature: "pub struct EditorComplete"
---

Floating autocomplete state modeled after classic RAD IDEs.

Floating autocomplete state (as in classic RAD IDEs).

# Examples

```rust
use crawrad_insight::list::EditorComplete;

let complete = EditorComplete::default();
assert!(!complete.aberto);
```
