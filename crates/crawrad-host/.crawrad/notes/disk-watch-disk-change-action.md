---
id: disk-watch-disk-change-action
kind: doc
target_file: src/disk_watch.rs
line_hint: 19
author: 
created: 
title: "disk_change_action"
tags: []
signature: "pub fn disk_change_action (disk : Option < & str > , ide_text : & str , has_buffer : bool , base : Option < & str > ,) -> DiskAction"
---

Pure decision for handling external file differences.

# Examples

```rust
use crawrad_host::disk_watch::{disk_change_action, DiskAction};

let action = disk_change_action(Some("text"), "text", false, None);
assert_eq!(action, DiskAction::Same);
```
