---
id: package-rows-row-template
kind: doc
target_file: src/package_rows.rs
line_hint: 64
author: 
created: 
title: "Row::template"
tags: []
signature: "pub fn template (& self) -> & 'static str"
---

The DSL template that draws this row.

The names are declared inside `packages_list` in the `live_design!` of
`ide_app.rs`, and a gate charges them against it — a name that does not
match draws NOTHING, with nothing failing.

# Examples

```rust
use crawrad_project::package_rows::Row;

assert_eq!(Row::Note("test".into()).template(), "Note");
```
