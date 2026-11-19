---
id: package-rows-row
kind: doc
target_file: src/package_rows.rs
line_hint: 40
author: 
created: 
title: "Row"
tags: []
signature: "pub enum Row"
---

One row of the panel.

# Examples

```rust
use crawrad_project::package_rows::Row;

let row = Row::Note("hello".into());
assert_eq!(row.template(), "Note");
```
