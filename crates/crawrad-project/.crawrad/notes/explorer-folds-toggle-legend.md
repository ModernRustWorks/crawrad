---
id: explorer-folds-toggle-legend
kind: doc
target_file: src/explorer.rs
line_hint: 179
author: 
created: 
title: "Folds::toggle_legend"
tags: []
signature: "pub fn toggle_legend (& mut self , document : Option < & Path > , active : Option < & Path >)"
---

Opens the legend when it is closed, and closes it when it is open.

# Examples

```rust
use crawrad_project::explorer::Folds;

let mut folds = Folds::default();
folds.toggle_legend(None, None);
assert!(folds.legend_open(None, None));
```
