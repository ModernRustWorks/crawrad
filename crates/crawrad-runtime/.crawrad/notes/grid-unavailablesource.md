---
id: grid-unavailablesource
kind: doc
target_file: src/grid.rs
line_hint: 553
author: 
created: 
title: "UnavailableSource"
tags: []
signature: "pub struct UnavailableSource"
---

A source that could not be opened, standing in for it so the grid says why.

A bound grid opens its table when the form starts; a wrong path or a
missing table must not leave a blank grid that looks like an empty table.
Every answer is the opening error, so it reaches the grid's footer the
way any other source error does.

# Examples

```rust
use crawrad_runtime::grid::UnavailableSource;

let src = UnavailableSource::new("file not found");
```
