---
id: explorer-rows-row-explorerrow-text
kind: doc
target_file: src/explorer_rows/row.rs
line_hint: 63
author: 
created: 
title: "ExplorerRow::text"
tags: []
signature: "fn text (& self) -> & str"
---

The text on the left.

# Examples

```rust
use crawrad_project::explorer_rows::row::ExplorerRow;

#[derive(Debug)]
struct DummyRow;
impl ExplorerRow for DummyRow {
    fn template(&self) -> &'static str { "Dummy" }
    fn text(&self) -> &str { "dummy" }
}
assert_eq!(DummyRow.text(), "dummy");
```
