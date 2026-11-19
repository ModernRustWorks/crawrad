---
id: chat-layout-code-block-scrolls
kind: doc
target_file: src/chat_layout.rs
line_hint: 23
author: 
created: 
title: "code_block_scrolls"
tags: []
signature: "pub fn code_block_scrolls (code : & str) -> bool"
---

Determines whether a code block scrolls inside the card.

Does this code block scroll inside the card instead of stretching it?

# Examples

```rust
use crawrad_agent::chat_layout::code_block_scrolls;

assert!(!code_block_scrolls("println!(\"hello\");"));
```
