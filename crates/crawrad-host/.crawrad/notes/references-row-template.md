---
id: references-row-template
kind: doc
target_file: src/references.rs
line_hint: 75
author: 
created: 
title: "Row::template"
tags: []
signature: "pub fn template (& self) -> & 'static str"
---

The list's template for the row.

# Examples

```rust
use crawrad_host::references::Row;

let row = Row::Hit {
    group: 0,
    hit: 0,
    line: "1".into(),
    before: "".into(),
    word: "name".into(),
    after: "".into(),
};
assert_eq!(row.template(), "RefHit");
```
