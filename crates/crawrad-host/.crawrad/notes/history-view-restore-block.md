---
id: history-view-restore-block
kind: doc
target_file: src/history_view.rs
line_hint: 141
author: 
created: 
title: "restore_block"
tags: []
signature: "pub fn restore_block (copy : & str , buffer : & str , block : usize) -> Option < String >"
---

Restores a single change block from copy into the buffer.

The buffer with block `block` (an index into `SideBySide::blocks`) put back as the copy has it;
every other line of the buffer kept. Keeps the buffer's line ending style (`\r\n` if the buffer
uses it) and whether it ends with a newline.

# Examples

```rust
use crawrad_host::history_view::restore_block;

let restored = restore_block("old\n", "new\n", 0);
assert_eq!(restored.as_deref(), Some("old\n"));
```
