---
id: chat-layout-message-count-label
kind: doc
target_file: src/chat_layout.rs
line_hint: 506
author: 
created: 
title: "message_count_label"
tags: []
signature: "pub fn message_count_label (count : usize) -> String"
---

The session row's count: `1 message`, `N messages`.

# Examples

```rust
use crawrad_agent::chat_layout::message_count_label;

assert_eq!(message_count_label(1), "1 message");
assert_eq!(message_count_label(5), "5 messages");
```
