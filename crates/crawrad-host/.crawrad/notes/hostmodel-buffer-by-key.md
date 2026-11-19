---
id: hostmodel-buffer-by-key
kind: doc
target_file: src/lib.rs
line_hint: 403
author: 
created: 
title: "HostModel::buffer_by_key"
tags: []
signature: "pub fn buffer_by_key (& self , key : & str) -> Option < & str >"
---

Returns the in-memory content whose canonical key equals `key`.

Returns the in-memory content whose canonical key equals `key`
(forward-slash path, stripped of leading `./`).

# Examples

```rust
use crawrad_host::HostModel;

let mut host = HostModel::default();
host.set_buffer("src/main.rs".into(), "fn main() {}".into());
assert_eq!(host.buffer_by_key("./src/main.rs"), Some("fn main() {}"));
```
