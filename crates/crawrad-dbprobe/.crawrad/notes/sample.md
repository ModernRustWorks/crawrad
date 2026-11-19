---
id: sample
kind: doc
target_file: src/lib.rs
line_hint: 6
author: 
created: 
title: "Sample"
tags: []
signature: "pub struct Sample"
---

A bounded sample of a database table for designer preview.

# Examples

```rust
use crawrad_dbprobe::Sample;

let sample = Sample {
    columns: vec!["id".to_string()],
    kinds: vec!["INTEGER".to_string()],
    rows: vec![vec![Some("1".to_string())]],
    total: 1,
};
assert_eq!(sample.total, 1);
```
