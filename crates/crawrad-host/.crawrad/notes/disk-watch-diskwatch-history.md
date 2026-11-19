---
id: disk-watch-diskwatch-history
kind: doc
target_file: src/disk_watch.rs
line_hint: 65
author: 
created: 
title: "DiskWatch::history"
tags: []
signature: "pub fn history (& self) -> Option < LocalHistory >"
---

Returns the local history instance for the watched root.

The local history of the watched workspace root; `None` without a
root or with the history turned off in Options.

# Examples

```rust
use crawrad_host::disk_watch::DiskWatch;

let dw = DiskWatch::default();
assert!(dw.history().is_none());
```
