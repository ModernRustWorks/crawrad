---
id: makepadruntime-set-design-rows
kind: doc
target_file: src/lib.rs
line_hint: 337
author: 
created: 
title: "MakepadRuntime::set_design_rows"
tags: []
signature: "pub fn set_design_rows( &mut self, rows: BTreeMap<NodeId, crawrad_ir::design_rows::GridRows>, ) -> bool"
---

Hands in the bound grids' design-time rows (#925); `true` when they changed.

# Examples

```rust
use std::collections::BTreeMap;
use crawrad_runtime_makepad::MakepadRuntime;

let mut rt = MakepadRuntime::new();
assert!(!rt.set_design_rows(BTreeMap::new()));
```
