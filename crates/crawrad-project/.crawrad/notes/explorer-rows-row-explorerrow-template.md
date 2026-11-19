---
id: explorer-rows-row-explorerrow-template
kind: doc
target_file: src/explorer_rows/row.rs
line_hint: 60
author: 
created: 
title: "ExplorerRow::template"
tags: []
signature: "fn template (& self) -> & 'static str"
---

The name of the DSL template that draws it.

# Examples

```rust
use crawrad_project::explorer_rows::row::ExplorerRow;

#[derive(Debug)]
struct DummyRow;
impl ExplorerRow for DummyRow {
    fn template(&self) -> &'static str { "Dummy" }
    fn text(&self) -> &str { "dummy" }
}
assert_eq!(DummyRow.template(), "Dummy");
```
