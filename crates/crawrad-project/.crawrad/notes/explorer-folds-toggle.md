---
id: explorer-folds-toggle
kind: doc
target_file: src/explorer.rs
line_hint: 171
author: 
created: 
title: "Folds::toggle"
tags: []
signature: "pub fn toggle (& mut self , document : Option < & Path > , active : Option < & Path > , folder : & Path)"
---

Opens what is closed and closes what is open.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer::Folds;

let mut folds = Folds::default();
folds.toggle(None, None, Path::new("folder"));
assert!(folds.open(None, None).contains(Path::new("folder")));
```
