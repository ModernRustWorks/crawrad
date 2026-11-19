---
id: explorer-folds-open
kind: doc
target_file: src/explorer.rs
line_hint: 139
author: 
created: 
title: "Folds::open"
tags: []
signature: "pub fn open (& mut self , document : Option < & Path > , active : Option < & Path >) -> & BTreeSet < PathBuf >"
---

The open folders for this document, rebuilding them when the document changed.

The open folders for this document, rebuilding them when the document
changed.

The fresh set holds the **active member** and nothing else, which is
the egui default (`.default_open(m.ativo)`): whoever opened the IDE is
working on one project, and the others are context.

# Examples

```rust
use crawrad_project::explorer::Folds;

let mut folds = Folds::default();
assert!(folds.open(None, None).is_empty());
```
