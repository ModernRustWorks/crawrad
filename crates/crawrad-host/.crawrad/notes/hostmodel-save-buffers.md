---
id: hostmodel-save-buffers
kind: doc
target_file: src/lib.rs
line_hint: 427
author: 
created: 
title: "HostModel::save_buffers"
tags: []
signature: "pub fn save_buffers (& mut self , write : & mut dyn FnMut (& Path , & str) -> Result < () , String > ,) -> Vec < (PathBuf , Result < () , String >) >"
---

Saves all dirty in-memory buffers through the given write callback.

**Save All's code half**: every unsaved edit goes through `write`
(the file and its text; the caller knows each file's encoding). A
written file stops being dirty; one whose write failed STAYS dirty,
so nothing typed is lost. The answer is each file and how it went,
in path order.

# Examples

```rust
use crawrad_host::HostModel;
use std::path::PathBuf;

let mut host = HostModel::default();
host.set_buffer(PathBuf::from("a.rs"), "code".into());
let report = host.save_buffers(&mut |_path, _text| Ok(()));
assert_eq!(report.len(), 1);
assert!(host.open_file_paths().is_empty());
```
