---
id: hostmodel-editor-buffers
kind: doc
target_file: src/lib.rs
line_hint: 455
author: 
created: 
title: "HostModel::editor_buffers"
tags: []
signature: "pub fn editor_buffers (& self) -> & BTreeMap < PathBuf , String >"
---

Returns a reference to the full editor-buffers map for bulk iteration.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::default();
assert!(host.editor_buffers().is_empty());
```
