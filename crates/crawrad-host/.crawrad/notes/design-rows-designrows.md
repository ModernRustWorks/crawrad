---
id: design-rows-designrows
kind: doc
target_file: src/design_rows.rs
line_hint: 74
author: 
created: 
title: "DesignRows"
tags: []
signature: "pub struct DesignRows"
---

The design-time rows of the open form's bound grids.

# Examples

```rust
use crawrad_host::design_rows::DesignRows;

let mut rows = DesignRows::default();
assert!(!rows.pump());
```
