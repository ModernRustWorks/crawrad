---
id: explorer-folds-collapse-all
kind: doc
target_file: src/explorer.rs
line_hint: 205
author: 
created: 
title: "Folds::collapse_all"
tags: []
signature: "pub fn collapse_all (& mut self , document : Option < & Path > , active : Option < & Path >)"
---

Closes all open folders and sections.

# Examples

```rust
use crawrad_project::explorer::Folds;

let mut folds = Folds::default();
folds.collapse_all(None, None);
assert!(!folds.legend_open(None, None));
```
