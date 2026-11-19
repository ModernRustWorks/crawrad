---
id: design-rows-thousands
kind: doc
target_file: src/design_rows.rs
line_hint: 96
author: 
created: 
title: "thousands"
tags: []
signature: "pub fn thousands(n: u64) -> String"
---

`1240` → `1,240`: the count as the footer writes it.

# Examples

```rust
use crawrad_ir::design_rows::thousands;

assert_eq!(thousands(1240), "1,240");
```
