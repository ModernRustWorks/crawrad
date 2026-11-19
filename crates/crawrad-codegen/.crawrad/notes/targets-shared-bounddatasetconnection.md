---
id: targets-shared-bounddatasetconnection
kind: doc
target_file: src/targets/shared.rs
line_hint: 139
author: 
created: 
title: "BoundDatasetConnection"
tags: []
signature: "pub struct BoundDatasetConnection"
---

The resolved database connection source for a dataset node (#788).

# Examples

```rust
use crawrad_codegen::targets::shared::BoundDatasetConnection;

let conn = BoundDatasetConnection {
    dataset_name: "dst1".to_string(),
    engine_name: "conn1".to_string(),
};
assert_eq!(conn.dataset_name, "dst1");
```
