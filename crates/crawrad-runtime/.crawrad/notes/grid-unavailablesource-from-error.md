---
id: grid-unavailablesource-from-error
kind: doc
target_file: src/grid.rs
line_hint: 3545
author: 
created: 
title: "UnavailableSource::from_error"
tags: []
signature: "pub fn from_error (error : & datagrid :: SourceError) -> Self"
---

The source that could not open, in the error's own words, said once.

# Examples

```rust
use crawrad_runtime::grid::UnavailableSource;

let e = datagrid::SourceError::Backend("database file not found: C:/p/a.db".into());
let _ = UnavailableSource::from_error(&e);
```
