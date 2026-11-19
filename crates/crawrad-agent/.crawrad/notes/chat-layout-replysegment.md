---
id: chat-layout-replysegment
kind: doc
target_file: src/chat_layout.rs
line_hint: 55
author: 
created: 
title: "ReplySegment"
tags: []
signature: "pub enum ReplySegment"
---

A single segment of an assistant reply.

A single segment of an assistant reply: either a prose text block or a fenced code block.

# Examples

```rust
use crawrad_agent::chat_layout::ReplySegment;

let seg = ReplySegment::Prose("Hello".to_string());
assert!(matches!(seg, ReplySegment::Prose(_)));
```
