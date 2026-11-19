---
id: hostmodel-editor-buffers-mut
kind: doc
target_file: src/lib.rs
line_hint: 460
author: 
created: 
title: "HostModel::editor_buffers_mut"
tags: []
signature: "pub fn editor_buffers_mut (& mut self) -> & mut BTreeMap < PathBuf , String >"
---

Returns a mutable reference to the full editor-buffers map.

# Examples

```rust
use crawrad_host::HostModel;

let mut host = HostModel::default();
host.editor_buffers_mut().insert("a.rs".into(), "text".into());
assert_eq!(host.editor_buffers().len(), 1);
```
