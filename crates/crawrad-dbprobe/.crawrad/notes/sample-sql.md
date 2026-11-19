---
id: sample-sql
kind: doc
target_file: src/lib.rs
line_hint: 104
author: 
created: 
title: "sample_sql"
tags: []
signature: "pub fn sample_sql (url : & str , sql : & str , limit : usize) -> Result < Sample , String >"
---

The first rows of a query, wrapped as a subquery so the probe only reads.

# Examples

```rust
use crawrad_dbprobe::sample_sql;

assert!(sample_sql("", "SELECT 1", 8).is_err());
```

# Errors

When the database cannot be opened or the query fails.
