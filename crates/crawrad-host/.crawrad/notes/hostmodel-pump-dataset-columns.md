---
id: hostmodel-pump-dataset-columns
kind: doc
target_file: src/lib.rs
line_hint: 182
author: 
created: 
title: "HostModel::pump_dataset_columns"
tags: []
signature: "pub fn pump_dataset_columns (& mut self) -> bool"
---

Takes in the probes' answers; `true` when the Inspector and canvas redraw.

# Examples

```rust
use crawrad_host::HostModel;

let mut host = HostModel::new();
assert!(!host.pump_dataset_columns());
```
