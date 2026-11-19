---
id: history-localhistory-content
kind: doc
target_file: src/history.rs
line_hint: 180
author: 
created: 
title: "LocalHistory::content"
tags: []
signature: "pub fn content (& self , file : & Path , entry : & Entry) -> Option < String >"
---

Returns the text content for the given history entry of `file`.

# Examples

```rust
use std::path::{Path, PathBuf};
use crawrad_host::history::{Entry, HistorySettings, LocalHistory, Origin};

let history = LocalHistory::new(PathBuf::from("."), HistorySettings::default());
let entry = Entry { time: 0, hash: "abc".into(), origin: Origin::Save };
assert!(history.content(Path::new("src/main.rs"), &entry).is_none());
```
