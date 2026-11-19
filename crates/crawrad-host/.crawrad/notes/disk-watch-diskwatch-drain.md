---
id: disk-watch-diskwatch-drain
kind: doc
target_file: src/disk_watch.rs
line_hint: 82
author: 
created: 
title: "DiskWatch::drain"
tags: []
signature: "pub fn drain (& mut self , open : & [PathBuf] , active : Option < & Path > , buffer : & dyn Fn (& Path) -> Option < String > , view_text : & str ,) -> (bool , Vec < DiskReaction >)"
---

Drains watcher events and returns reactions for open files.

Drains the watcher. Returns whether any file changed, and reactions for open files.

# Examples

```rust
use crawrad_host::disk_watch::DiskWatch;

let mut dw = DiskWatch::default();
let (changed, reactions) = dw.drain(&[], None, &|_| None, "");
assert!(!changed);
assert!(reactions.is_empty());
```
