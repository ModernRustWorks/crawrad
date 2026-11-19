---
id: history-localhistory
kind: doc
target_file: src/history.rs
line_hint: 70
author: 
created: 
title: "LocalHistory"
tags: []
signature: "pub struct LocalHistory"
---

Local file history engine managing snapshots under the workspace root.

Local file history engine managing snapshots under the workspace root (`.crawrad/history`).

# Examples

```rust
use std::path::PathBuf;
use crawrad_host::history::{HistorySettings, LocalHistory};

let _history = LocalHistory::new(PathBuf::from("."), HistorySettings::default());
```
