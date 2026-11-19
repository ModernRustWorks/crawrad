---
id: dataset-columns-make-url-absolute
kind: doc
target_file: src/dataset_columns.rs
line_hint: 48
author: 
created: 
title: "make_url_absolute"
tags: []
signature: "pub fn make_url_absolute (url : & str , project_dir : Option < & Path >) -> String"
---

A SQLite url naming a relative file, made absolute against the project.

The generated app opens `sqlite://shop.db` from the folder it runs in, which
is the project's; the IDE runs elsewhere, so it resolves the same file
itself. Any other url (another database, `:memory:`, a `file:` URI, an
absolute path) is returned as it is.

# Examples

```rust
use std::path::Path;
use crawrad_host::dataset_columns::make_url_absolute;

let abs = make_url_absolute("sqlite://app.db", Some(Path::new("/tmp")));
assert!(abs.starts_with("sqlite://"));
```
