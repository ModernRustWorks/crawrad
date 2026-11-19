---
id: design-rows-probe-rows
kind: doc
target_file: src/design_rows.rs
line_hint: 256
author: 
created: 
title: "probe_rows"
tags: []
signature: "pub fn probe_rows(exe: &Path, url: &str, query: &RowQuery, limit: usize) -> Answer"
---

Asks `crawrad-dbprobe` for the columns and the first rows of a table or query.

# Examples

```rust
use std::path::Path;
use crawrad_host::design_rows::{RowQuery, probe_rows};

let t = RowQuery::Table("t".into());
assert!(probe_rows(Path::new("no-such-probe"), "sqlite://a.db", &t, 8).is_err());
```

# Errors

When the probe cannot start, takes longer than the time limit, or answers an error.
