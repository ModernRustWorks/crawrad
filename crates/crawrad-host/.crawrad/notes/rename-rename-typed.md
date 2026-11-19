---
id: rename-rename-typed
kind: doc
target_file: src/rename.rs
line_hint: 169
author: 
created: 
title: "Rename::typed"
tags: []
signature: "pub fn typed (& mut self , new : & str)"
---

The new name was typed: back to naming (an old preview is stale).

# Examples

```rust
use crawrad_host::rename::Rename;

let mut rename = Rename::default();
rename.typed("new_name");
assert_eq!(rename.new, "new_name");
```
