---
id: hostmodel-is-dirty
kind: doc
target_file: src/lib.rs
line_hint: 345
author: 
created: 
title: "HostModel::is_dirty"
tags: []
signature: "pub fn is_dirty (& self) -> bool"
---

Returns `true` if there are unsaved edits in any open file.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(!host.is_dirty());
```
