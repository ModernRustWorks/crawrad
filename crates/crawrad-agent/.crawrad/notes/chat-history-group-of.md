---
id: chat-history-group-of
kind: doc
target_file: src/chat_history.rs
line_hint: 83
author: 
created: 
title: "group_of"
tags: []
signature: "pub fn group_of (stamp : & str , today : & str) -> & 'static str"
---

`"Today"`, `"Yesterday"` or `"Earlier"` for a `"YYYY-MM-DD HH:MM"` stamp.

# Examples

```rust
use crawrad_agent::chat_history::group_of;

assert_eq!(group_of("2026-11-05 10:00", "2026-11-05"), "Today");
```
