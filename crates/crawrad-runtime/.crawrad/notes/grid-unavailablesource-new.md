---
id: grid-unavailablesource-new
kind: doc
target_file: src/grid.rs
line_hint: 559
author: 
created: 
title: "UnavailableSource::new"
tags: []
signature: "pub fn new (message : impl Into < String >) -> Self"
---

A source whose every query fails with `message`.

# Examples

```rust
use crawrad_runtime::grid::UnavailableSource;

let src = UnavailableSource::new("database unreachable");
```
