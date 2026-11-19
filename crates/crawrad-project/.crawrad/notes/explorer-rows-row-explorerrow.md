---
id: explorer-rows-row-explorerrow
kind: doc
target_file: src/explorer_rows/row.rs
line_hint: 58
author: 
created: 
title: "ExplorerRow"
tags: []
signature: "pub trait ExplorerRow : Debug"
---

One row of the panel.

Everything has a default except [`Self::template`] and [`Self::text`]: a
row that forgets its icon draws without one and still reads, but a row that
forgets its template draws nothing at all.

# Examples

```rust
use crawrad_project::explorer_rows::row::ExplorerRow;

#[derive(Debug)]
struct DummyRow;
impl ExplorerRow for DummyRow {
    fn template(&self) -> &'static str { "Dummy" }
    fn text(&self) -> &str { "dummy" }
}
let row = DummyRow;
assert_eq!(row.text(), "dummy");
```
