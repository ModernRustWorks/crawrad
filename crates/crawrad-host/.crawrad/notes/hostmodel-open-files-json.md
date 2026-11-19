---
id: hostmodel-open-files-json
kind: doc
target_file: src/lib.rs
line_hint: 376
author: 
created: 
title: "HostModel::open_files_json"
tags: []
signature: "pub fn open_files_json (& self) -> serde_json :: Value"
---

`editor.list_open_files`: every buffer, the active file flagged.

# Examples

```rust
use crawrad_host::HostModel;
use std::path::PathBuf;

let mut host = HostModel::default();
host.set_buffer(PathBuf::from("a.rs"), "code".into());
let json = host.open_files_json();
assert_eq!(json["count"], 1);
```
