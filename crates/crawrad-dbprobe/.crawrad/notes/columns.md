---
id: columns
kind: doc
target_file: src/lib.rs
line_hint: 6
author: 
created: 
title: "columns"
tags: []
signature: "pub fn columns (url : & str , table : & str) -> Result < Vec < String > , String >"
---

Discovers column names in order for a given database URL and table name.

# Examples

```rust
use crawrad_dbprobe::columns;

let res = columns("", "users");
assert!(res.is_err());
```

# Errors

Returns an error string if URL or table is empty, database file is missing, or inspection fails.
