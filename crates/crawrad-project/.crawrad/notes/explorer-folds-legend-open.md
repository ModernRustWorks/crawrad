---
id: explorer-folds-legend-open
kind: doc
target_file: src/explorer.rs
line_hint: 145
author: 
created: 
title: "Folds::legend_open"
tags: []
signature: "pub fn legend_open (& mut self , document : Option < & Path > , active : Option < & Path >) -> bool"
---

Returns whether the legend section is currently open.

Is the legend section open?

It takes the document for the same reason [`Folds::open`] does: read
against stale state, it would answer with the legend of another
document.

# Examples

```rust
use crawrad_project::explorer::Folds;

let mut folds = Folds::default();
assert!(!folds.legend_open(None, None));
```
