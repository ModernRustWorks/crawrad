---
id: db-url-resolve-sqlite-url
kind: doc
target_file: src/db_url.rs
line_hint: 36
author: 
created: 
title: "resolve_sqlite_url"
tags: []
signature: "pub fn resolve_sqlite_url (url : & str , project_dir : & str) -> String"
---

A relative SQLite file, found by the exe or in the project; never the cwd.

# Examples

```rust
use crawrad_runtime::db_url::resolve_sqlite_url;

assert_eq!(resolve_sqlite_url("sqlite::memory:", "."), "sqlite::memory:");
```
