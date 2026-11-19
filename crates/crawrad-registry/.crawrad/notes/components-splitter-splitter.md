---
id: components-splitter-splitter
kind: doc
target_file: src/components/splitter.rs
line_hint: 5
author: 
created: 
title: "splitter"
tags: []
signature: "pub fn splitter () -> ComponentDesc"
---

`Splitter`: dynamic runtime resize divider for aligned controls.

# Examples

```rust
use crawrad_registry::splitter;

let desc = splitter();
assert_eq!(desc.class, "Splitter");
```
