---
id: hostmodel-active-file
kind: doc
target_file: src/lib.rs
line_hint: 331
author: 
created: 
title: "HostModel::active_file"
tags: []
signature: "pub fn active_file (& self) -> Option < & Path >"
---

Returns the path of the file currently active in the editor, if any.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert_eq!(host.active_file(), None);
```
