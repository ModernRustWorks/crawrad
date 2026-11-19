---
id: inspector-model-no-descriptor
kind: doc
target_file: src/inspector_model.rs
line_hint: 140
author: 
created: 
title: "NO_DESCRIPTOR"
tags: []
signature: "pub const NO_DESCRIPTOR : & str = \"component has no descriptor in the registry\""
---

The class of the selected component is not in the catalogue (`inspector.rs:92`).

# Examples

```rust
use crawrad_actions::inspector_model::NO_DESCRIPTOR;

assert!(NO_DESCRIPTOR.contains("no descriptor"));
```
