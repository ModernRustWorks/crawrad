---
id: chat-history-when-of
kind: doc
target_file: src/chat_history.rs
line_hint: 92
author: 
created: 
title: "when_of"
tags: []
signature: "pub fn when_of (stamp : & str , today : & str) -> String"
---

The time for today and yesterday (`"08:31"`), the date before (`"Oct 4"`).

# Examples

```rust
use crawrad_agent::chat_history::when_of;

assert_eq!(when_of("2026-11-05 10:00", "2026-11-05"), "10:00");
```
