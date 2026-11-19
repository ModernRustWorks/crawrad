---
id: design-rows-rowquery
kind: doc
target_file: src/design_rows.rs
line_hint: 30
author: 
created: 
title: "RowQuery"
tags: []
signature: "pub enum RowQuery"
---

What the design-time probe reads: a whole table, or a dataset's query.

# Examples

```rust
use crawrad_host::design_rows::RowQuery;

let q = RowQuery::Sql("SELECT name FROM contacts".into());
assert_ne!(q, RowQuery::Table("contacts".into()));
```
