---
id: db-url-resolve-with
kind: doc
target_file: src/db_url.rs
line_hint: 45
author: 
created: 
title: "resolve_with"
tags: []
signature: "pub fn resolve_with (url : & str , exe_dir : Option < & Path > , project_dir : & Path) -> String"
---

resolve_sqlite_url with the executable folder given, for tests.

# Examples

```rust
use std::path::Path;
use crawrad_runtime::db_url::resolve_with;

assert_eq!(resolve_with("postgres://h/db", None, Path::new(".")), "postgres://h/db");
```
