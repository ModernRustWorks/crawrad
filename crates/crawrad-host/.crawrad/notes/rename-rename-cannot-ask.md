---
id: rename-rename-cannot-ask
kind: doc
target_file: src/rename.rs
line_hint: 178
author: 
created: 
title: "Rename::cannot_ask"
tags: []
signature: "pub fn cannot_ask (& self) -> Option < & 'static str >"
---

Why Enter cannot ask yet, or `None` when the new name may be asked.

# Examples

```rust
use crawrad_host::rename::Rename;

let rename = Rename::default();
assert_eq!(rename.cannot_ask(), Some("Type the new name"));
```
