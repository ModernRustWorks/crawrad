---
id: chat-history-row
kind: doc
target_file: src/chat_history.rs
line_hint: 242
author: 
created: 
title: "Row"
tags: []
signature: "pub struct Row"
---

One drawn row of the history list (#478, board 4 of the chat mockup).

# Examples

```rust
use crawrad_agent::chat_history::Row;

let r = Row {
    id: "c1".to_string(),
    group: Some("Today".to_string()),
    title: "Test".to_string(),
    meta: "Claude · 1 turn".to_string(),
    when: "10:00".to_string(),
    resume: "resumes",
    open: true,
};
assert!(r.open);
```
