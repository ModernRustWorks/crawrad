---
id: references-row
kind: doc
target_file: src/references.rs
line_hint: 50
author: 
created: 
title: "Row"
tags: []
signature: "pub enum Row"
---

A row of the panel's list.

# Examples

```rust
use crawrad_host::references::Row;

let row = Row::File {
    group: 0,
    name: "lib.rs".into(),
    folder: "src".into(),
    count: 1,
    mark: None,
    open: true,
};
assert_eq!(row.template(), "RefFile");
```
