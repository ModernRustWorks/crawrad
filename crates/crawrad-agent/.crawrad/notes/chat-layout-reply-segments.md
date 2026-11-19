---
id: chat-layout-reply-segments
kind: doc
target_file: src/chat_layout.rs
line_hint: 74
author: 
created: 
title: "reply_segments"
tags: []
signature: "pub fn reply_segments (reply_text : & str) -> Vec < ReplySegment >"
---

Parses reply markdown into prose and code segments.

Parses assistant reply markdown into an ordered sequence of prose and code segments.

Consecutive text blocks are joined with "\n\n". A reply without code blocks
returns a single `Prose(reply_text)` segment.

# Examples

```rust
use crawrad_agent::chat_layout::reply_segments;

let segs = reply_segments("Hello world");
assert_eq!(segs.len(), 1);
```
