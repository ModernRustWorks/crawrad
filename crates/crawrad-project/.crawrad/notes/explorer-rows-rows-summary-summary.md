---
id: explorer-rows-rows-summary-summary
kind: doc
target_file: src/explorer_rows/rows/summary.rs
line_hint: 9
author: 
created: 
title: "Summary"
tags: []
signature: "pub struct Summary"
---

The count line.

# Examples

```rust
use crawrad_project::explorer_rows::rows::summary::Summary;
use crawrad_project::explorer_rows::row::ExplorerRow;

let s = Summary::new("4 files", 0);
assert_eq!(s.text(), "4 files");
```
