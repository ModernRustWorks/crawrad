---
id: chat-history-rows
kind: doc
target_file: src/chat_history.rs
line_hint: 258
author: 
created: 
title: "rows"
tags: []
signature: "pub fn rows (summaries : & [Value] , open : Option < & str > , max : usize) -> Vec < Row >"
---

The rows the list draws from `chat.history`'s summaries, at most `max`.

# Examples

```rust
use crawrad_agent::chat_history::rows;

let r = rows(&[], None, 10);
assert!(r.is_empty());
```
