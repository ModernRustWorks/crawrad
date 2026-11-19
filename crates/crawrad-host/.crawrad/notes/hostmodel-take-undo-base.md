---
id: hostmodel-take-undo-base
kind: doc
target_file: src/lib.rs
line_hint: 450
author: 
created: 
title: "HostModel::take_undo_base"
tags: []
signature: "pub fn take_undo_base (& mut self , path : & Path) -> Option < String >"
---

Takes and removes the recorded undo base text for a file.

The text from before a rename, once: the editor shows it, then the
edit on top as one undo step.

# Examples

```rust
use crawrad_host::HostModel;
use std::path::Path;

let mut host = HostModel::default();
host.set_buffer_undoable("a.rs".into(), "old".into(), "new".into());
assert_eq!(host.take_undo_base(Path::new("a.rs")), Some("old".into()));
assert_eq!(host.take_undo_base(Path::new("a.rs")), None);
```
