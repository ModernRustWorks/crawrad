---
id: sample-rows
kind: doc
target_file: src/lib.rs
line_hint: 40
author: 
created: 
title: "sample"
tags: []
signature: "pub fn sample (url : & str , table : & str , limit : usize) -> Result < Sample , String >"
---

Returns column names, preview rows, and total row count for a table.

# Examples

```rust
use crawrad_dbprobe::sample;

let res = sample("", "users", 10);
assert!(res.is_err());
```

# Errors

Returns an error string if URL or table is empty, database file is missing, or query fails.
