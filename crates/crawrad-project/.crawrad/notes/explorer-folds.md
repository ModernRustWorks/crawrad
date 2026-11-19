---
id: explorer-folds
kind: doc
target_file: src/explorer.rs
line_hint: 118
author: 
created: 
title: "Folds"
tags: []
signature: "pub struct Folds"
---

What is open in the panel, tied to the document it is about.

The document is stored **with** the state on purpose — see the module doc.
Reading through [`Folds::open`] is the only way in, and it cannot answer
with the folds of another document.

# Examples

```rust
use crawrad_project::explorer::Folds;

let mut folds = Folds::default();
assert!(!folds.legend_open(None, None));
```
