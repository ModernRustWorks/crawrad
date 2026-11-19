---
id: chat-layout-user-bubble-min-indent
kind: doc
target_file: src/chat_layout.rs
line_hint: 4
author: 
created: 
title: "USER_BUBBLE_MIN_INDENT"
tags: []
signature: "pub const USER_BUBBLE_MIN_INDENT : f64 = 24.0"
---

The minimum left indentation for a user message bubble.

The least room the user's bubble leaves on its left; a bubble that would
need less wraps its text instead.

# Examples

```rust
use crawrad_agent::chat_layout::USER_BUBBLE_MIN_INDENT;

assert!(USER_BUBBLE_MIN_INDENT > 0.0);
```
