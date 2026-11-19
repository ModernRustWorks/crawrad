---
id: history-localhistory-record
kind: doc
target_file: src/history.rs
line_hint: 87
author: 
created: 
title: "LocalHistory::record"
tags: []
signature: "pub fn record (& self , file : & Path , text : & str , origin : Origin , time : u64 ,) -> std :: io :: Result < bool >"
---

Records a new revision if the content differs from the latest entry.

Returns `Ok(true)` if recorded, `Ok(false)` if outside root or deduplicated.

# Examples

```rust,no_run
// Writes snapshot files under .crawrad/history.
use std::path::{Path, PathBuf};
use crawrad_host::history::{HistorySettings, LocalHistory, Origin};

let history = LocalHistory::new(PathBuf::from("."), HistorySettings::default());
let res = history.record(Path::new("src/main.rs"), "code", Origin::Save, 100);
assert!(res.is_ok());
```

# Errors

Returns an I/O error if creating directories or writing history entry files fails.
