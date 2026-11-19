---
id: link-locate-handler
kind: doc
target_file: src/link.rs
line_hint: 73
author: 
created: 
title: "locate_handler"
tags: []
signature: "pub fn locate_handler (fonte : & str , nome : & str) -> Option < Local >"
---

Where an `fn` **begins and ends** in the file — what pruning needs to slice out.

# Examples

```rust
use crawrad_codegen::link::locate_handler;

let src = "fn test() {}";
assert!(locate_handler(src, "test").is_some());
```
