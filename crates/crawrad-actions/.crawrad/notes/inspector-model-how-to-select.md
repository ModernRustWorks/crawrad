---
id: inspector-model-how-to-select
kind: doc
target_file: src/inspector_model.rs
line_hint: 137
author: 
created: 
title: "HOW_TO_SELECT"
tags: []
signature: "pub const HOW_TO_SELECT : & str = \"Click a component on the form.\""
---

And the line of help that says what to do, also from egui (`inspector.rs:40`).

# Examples

```rust
use crawrad_actions::inspector_model::HOW_TO_SELECT;

assert_eq!(HOW_TO_SELECT, "Click a component on the form.");
```
