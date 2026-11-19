---
id: explorer-rows-rows-legend-legend
kind: doc
target_file: src/explorer_rows/rows/legend.rs
line_hint: 14
author: 
created: 
title: "Legend"
tags: []
signature: "pub struct Legend"
---

One pair of the legend.

# Examples

```rust
use crawrad_project::explorer_rows::rows::legend::Legend;
use crawrad_project::explorer_rows::row::ExplorerRow;

let leg = Legend::new("active", "Generate and Run write here", false);
assert_eq!(leg.text(), "active");
```
