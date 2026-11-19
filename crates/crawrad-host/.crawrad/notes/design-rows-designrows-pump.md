---
id: design-rows-designrows-pump
kind: doc
target_file: src/design_rows.rs
line_hint: 146
author: 
created: 
title: "DesignRows::pump"
tags: []
signature: "pub fn pump(&mut self) -> bool"
---

Takes in the reads that finished; `true` when something changed.

# Examples

```rust
use crawrad_host::design_rows::DesignRows;

let mut rows = DesignRows::default();
assert!(!rows.pump());
```
