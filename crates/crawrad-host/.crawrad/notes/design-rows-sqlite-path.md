---
id: design-rows-sqlite-path
kind: doc
target_file: src/design_rows.rs
line_hint: 57
author: 
created: 
title: "sqlite_path"
tags: []
signature: "pub fn sqlite_path(url: &str) -> Option<PathBuf>"
---

The database file behind a `sqlite:` url, when there is one.

# Examples

```rust
use crawrad_host::design_rows::sqlite_path;

assert!(sqlite_path("sqlite://app.db").is_some());
assert!(sqlite_path("sqlite::memory:").is_none());
```
