---
id: chat-layout-mention-token
kind: doc
target_file: src/chat_layout.rs
line_hint: 231
author: 
created: 
title: "mention_token"
tags: []
signature: "pub fn mention_token (active : Option < & std :: path :: Path > , root : Option < & std :: path :: Path >) -> String"
---

Generates the mention token for the currently active file.

What `Mentions` adds: the open file, relative to the workspace (`@src/main.rs `), or `@`.

# Examples

```rust
use crawrad_agent::chat_layout::mention_token;

assert_eq!(mention_token(None, None), "@");
```
