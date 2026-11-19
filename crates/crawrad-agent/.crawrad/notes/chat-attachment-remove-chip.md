---
id: chat-attachment-remove-chip
kind: doc
target_file: src/chat_attachment.rs
line_hint: 193
author: 
created: 
title: "remove_chip"
tags: []
signature: "pub fn remove_chip (selections : & mut Vec < SelectionAttachment > , images : & mut Vec < ImageAttachment > , i : usize)"
---

Removes chip `i`: the selections come first, then the pictures.

An index past the last chip removes nothing.

# Examples

```rust
use crawrad_agent::{ImageAttachment, remove_chip};

let mut images = vec![ImageAttachment { path: "a.png".into(), width: 1, height: 1 }];
remove_chip(&mut Vec::new(), &mut images, 0);
assert!(images.is_empty());
```
