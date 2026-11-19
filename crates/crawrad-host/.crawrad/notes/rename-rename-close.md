---
id: rename-rename-close
kind: doc
target_file: src/rename.rs
line_hint: 211
author: 
created: 
title: "Rename::close"
tags: []
signature: "pub fn close (& mut self)"
---

The dialog closes; nothing changes.

# Examples

```rust
use crawrad_host::rename::Rename;

let mut rename = Rename::default();
rename.close();
assert!(!rename.is_open());
```
