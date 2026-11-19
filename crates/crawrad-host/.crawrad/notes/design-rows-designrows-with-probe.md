---
id: design-rows-designrows-with-probe
kind: doc
target_file: src/design_rows.rs
line_hint: 94
author: 
created: 
title: "DesignRows::with_probe"
tags: []
signature: "pub fn with_probe( probe: fn() -> Option<PathBuf>, run: fn(&Path, &str, &RowQuery, usize) -> Answer, ) -> Self"
---

A cache that finds the probe with `probe` and runs it with `run`.

# Examples

```rust
use crawrad_host::design_rows::DesignRows;

let mut rows = DesignRows::with_probe(|| None, |_, _, _, _| Err("no".into()));
assert!(!rows.pump());
```
