---
id: db-url-sqlite-file
kind: doc
target_file: src/db_url.rs
line_hint: 24
author: 
created: 
title: "sqlite_file"
tags: []
signature: "pub fn sqlite_file (url : & str) -> Option < & str >"
---

The file a SQLite url names, or None for in-memory and URI forms.

# Examples

```rust
use crawrad_runtime::db_url::sqlite_file;

assert_eq!(sqlite_file("sqlite:contacts.db"), Some("contacts.db"));
assert_eq!(sqlite_file("sqlite::memory:"), None);
```
