---
id: design-rows-sample
kind: doc
target_file: src/design_rows.rs
line_hint: 18
author: 
created: 
title: "Sample"
tags: []
signature: "pub struct Sample"
---

The rows read from a table: its columns, the first rows as text, the count.

# Examples

```rust
use crawrad_ir::design_rows::Sample;

let s = Sample::default();
assert_eq!(s.total, 0);
```
