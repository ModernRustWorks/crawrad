---
id: chat-attachment-text-hash
kind: doc
target_file: src/chat_attachment.rs
line_hint: 25
author: 
created: 
title: "text_hash"
tags: []
signature: "pub fn text_hash (text : & str) -> u64"
---

Computes a 64-bit hash of the provided text.

# Examples

```rust
use crawrad_agent::chat_attachment::text_hash;

let h1 = text_hash("hello");
let h2 = text_hash("hello");
assert_eq!(h1, h2);
```
