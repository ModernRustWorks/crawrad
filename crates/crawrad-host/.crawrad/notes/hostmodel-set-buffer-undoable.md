---
id: hostmodel-set-buffer-undoable
kind: doc
target_file: src/lib.rs
line_hint: 444
author: 
created: 
title: "HostModel::set_buffer_undoable"
tags: []
signature: "pub fn set_buffer_undoable (& mut self , path : PathBuf , before : String , after : String)"
---

Sets an in-memory buffer while recording its prior text for undo.

A rename's edit to a file that is not in front: the new text is the
file's unsaved edit, and the text from before is kept for its undo
(the first one kept wins: two renames undo to the original).

# Examples

```rust
use crawrad_host::HostModel;
use std::path::Path;

let mut host = HostModel::default();
host.set_buffer_undoable("a.rs".into(), "old".into(), "new".into());
assert_eq!(host.take_undo_base(Path::new("a.rs")), Some("old".into()));
```
