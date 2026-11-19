---
id: chat-layout-delivery-mark
kind: doc
target_file: src/chat_layout.rs
line_hint: 28
author: 
created: 
title: "delivery_mark"
tags: []
signature: "pub fn delivery_mark (reply_started : bool) -> & 'static str"
---

Returns the delivery mark icon glyph (Codicons) for a user message bubble.

Returns `"\u{eab2}"` (Codicons `check`) when sent (not yet replied),
and `"\u{ebb1}"` (Codicons `check-all`) when the agent reply has started.

# Examples

```rust
use crawrad_agent::chat_layout::delivery_mark;

assert_eq!(delivery_mark(false), "\u{eab2}");
assert_eq!(delivery_mark(true), "\u{ebb1}");
```
