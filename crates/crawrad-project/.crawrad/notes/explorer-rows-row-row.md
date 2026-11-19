---
id: explorer-rows-row-row
kind: doc
target_file: src/explorer_rows/row.rs
line_hint: 102
author: 
created: 
title: "Row"
tags: []
signature: "pub type Row = Box < dyn ExplorerRow >"
---

One row, owned.

`Box<dyn>` and not an `enum`: see the module doc. The cost is one
allocation per row per frame, over a list that is tens of rows long and
already rebuilt from disk every draw — the panel recomputes the generation
plan on the same pass, which is microseconds and orders of magnitude more
expensive than this.

# Examples

```rust
use crawrad_project::explorer_rows::row::{ExplorerRow, Row};

#[derive(Debug)]
struct DummyRow;
impl ExplorerRow for DummyRow {
    fn template(&self) -> &'static str { "Dummy" }
    fn text(&self) -> &str { "dummy" }
}
let row: Row = Box::new(DummyRow);
assert_eq!(row.template(), "Dummy");
```
