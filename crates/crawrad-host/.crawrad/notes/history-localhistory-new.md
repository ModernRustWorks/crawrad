---
id: history-localhistory-new
kind: doc
target_file: src/history.rs
line_hint: 81
author: 
created: 
title: "LocalHistory::new"
tags: []
signature: "pub fn new (root : PathBuf , mut settings : HistorySettings) -> Self"
---

Creates a new history engine clamped to 1..=100 entries per file.

# Examples

```rust
use std::path::PathBuf;
use crawrad_host::history::{HistorySettings, LocalHistory};

let _history = LocalHistory::new(PathBuf::from("."), HistorySettings::default());
```
