---
id: inspector-model-tab-label
kind: doc
target_file: src/inspector_model.rs
line_hint: 104
author: 
created: 
title: "Tab::label"
tags: []
signature: "pub fn label (self) -> & 'static str"
---

The label, in English like the whole interface.

# Examples

```rust
use crawrad_actions::inspector_model::Tab;

assert_eq!(Tab::Properties.label(), "Properties");
```
