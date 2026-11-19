---
id: chat-layout-append-token
kind: doc
target_file: src/chat_layout.rs
line_hint: 221
author: 
created: 
title: "append_token"
tags: []
signature: "pub fn append_token (text : & str , token : & str) -> String"
---

The prompt's text with `token` appended, one space apart.

# Examples

```rust
use crawrad_agent::chat_layout::append_token;

assert_eq!(append_token("hello", "world"), "hello world");
```
