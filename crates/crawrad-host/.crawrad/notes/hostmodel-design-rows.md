---
id: hostmodel-design-rows
kind: doc
target_file: src/lib.rs
line_hint: 194
author: 
created: 
title: "HostModel::design_rows"
tags: []
signature: "pub fn design_rows( &self, project_dir: Option<&Path>, ) -> BTreeMap<crawrad_ir::NodeId, crawrad_ir::design_rows::GridRows>"
---

What each bound grid of the open form shows at design time, by node.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(host.design_rows(None).is_empty());
```
