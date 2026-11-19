---
id: history-localhistory-default-limit
kind: doc
target_file: src/history.rs
line_hint: 78
author: 
created: 
title: "LocalHistory::DEFAULT_LIMIT"
tags: []
signature: "pub const DEFAULT_LIMIT : usize = 10"
---

Default maximum entries stored per file.

# Examples

```rust
use crawrad_host::history::LocalHistory;

assert_eq!(LocalHistory::DEFAULT_LIMIT, 10);
```
