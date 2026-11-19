---
id: dock-encode
kind: doc
target_file: src/dock.rs
line_hint: 23
author: 
created: 
title: "encode"
tags: []
signature: "pub fn encode (items : & HashMap < LiveId , DockItem >) -> String"
---

The arrangement as text, ready to be written to disk.

# Examples

```rust
use std::collections::HashMap;
use crawrad_runtime_makepad::dock::encode;

let text = encode(&HashMap::new());
assert!(text.starts_with("crawrad-dock-layout"));
```
