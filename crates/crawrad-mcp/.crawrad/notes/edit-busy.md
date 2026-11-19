---
id: edit-busy
kind: doc
target_file: src/edit.rs
line_hint: 11
author: 
created: 
title: "busy"
tags: []
signature: "pub fn busy (path : & str) -> String"
---

The sentence for an edit sent while a review waits on the same file.

# Examples

```rust
use crawrad_mcp::edit::busy;

let msg = busy("src/main.rs");
assert!(msg.contains("src/main.rs"));
```
