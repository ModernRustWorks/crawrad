---
id: targets-shared-bound-dataset-open
kind: doc
target_file: src/targets/shared.rs
line_hint: 182
author: 
created: 
title: "bound_dataset_open"
tags: []
signature: "pub fn bound_dataset_open (engine_url_expr : & str , dataset_table_expr : & str) -> String"
---

Expression opening a bound dataset table through `PortaDbBridge`.

The expression that opens a bound dataset table:
`crawrad_binding::PortaDbBridge::open_table({engine_url_expr}, {dataset_table_expr})`.

# Examples

```rust
use crawrad_codegen::targets::shared::bound_dataset_open;

let expr = bound_dataset_open("&self.dbe1_url", "&self.dst1_table");
assert!(expr.contains("PortaDbBridge::open_table"));
```
