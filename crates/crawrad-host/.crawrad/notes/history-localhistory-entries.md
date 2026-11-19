---
id: history-localhistory-entries
kind: doc
target_file: src/history.rs
line_hint: 173
author: 
created: 
title: "LocalHistory::entries"
tags: []
signature: "pub fn entries (& self , file : & Path) -> Vec < Entry >"
---

Returns recorded entries for `file`, oldest first.

Returns recorded entries for `file`, oldest first. Empty when none or unreadable.

# Examples

```rust
use std::path::{Path, PathBuf};
use crawrad_host::history::{HistorySettings, LocalHistory};

let history = LocalHistory::new(PathBuf::from("."), HistorySettings::default());
assert!(history.entries(Path::new("src/main.rs")).is_empty());
```
