---
id: design-rows-gridsource
kind: doc
target_file: src/design_rows.rs
line_hint: 39
author: 
created: 
title: "GridSource"
tags: []
signature: "pub struct GridSource"
---

Where a bound `Grid` reads its design-time rows from.

# Examples

```rust
use crawrad_host::design_rows::{GridSource, RowQuery};

let s = GridSource {
    url: "sqlite://app.db".into(),
    query: RowQuery::Table("t".into()),
    table: "t".into(),
    limit: 8,
};
assert_eq!(s.limit, 8);
```
