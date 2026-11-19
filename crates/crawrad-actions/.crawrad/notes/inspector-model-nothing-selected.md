---
id: inspector-model-nothing-selected
kind: doc
target_file: src/inspector_model.rs
line_hint: 134
author: 
created: 
title: "NOTHING_SELECTED"
tags: []
signature: "pub const NOTHING_SELECTED : & str = \"Nothing selected.\""
---

Notice shown when a form is open and nothing is selected.

There is a form and nothing is selected — the same sentence as the egui
shell's (`crates/crawrad-ide/src/inspector.rs:38`).

# Examples

```rust
use crawrad_actions::inspector_model::NOTHING_SELECTED;

assert_eq!(NOTHING_SELECTED, "Nothing selected.");
```
