---
id: targets-shared-selection
kind: doc
target_file: src/targets/shared.rs
line_hint: 476
author: 
created: 
title: "Selection"
tags: []
signature: "pub enum Selection"
---

How a list control keeps the row it has selected (#88).

# Examples

```rust
use crawrad_codegen::targets::shared::Selection;

let sel = Selection::Optional;
assert!(matches!(sel, Selection::Optional));
```
