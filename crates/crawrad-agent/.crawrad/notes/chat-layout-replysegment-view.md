---
id: chat-layout-replysegment-view
kind: doc
target_file: src/chat_layout.rs
line_hint: 65
author: 
created: 
title: "ReplySegment::view"
tags: []
signature: "pub fn view (& self) -> (Option < & str > , & str , bool)"
---

Returns the view representation of this reply segment.

How a view draws it: the code's language (`None` for prose), the text,
and whether a code block is tall enough to scroll ([`code_block_scrolls`]).

# Examples

```rust
use crawrad_agent::chat_layout::ReplySegment;

let seg = ReplySegment::Prose("Hello".to_string());
let (lang, text, scroll) = seg.view();
assert_eq!(lang, None);
assert_eq!(text, "Hello");
assert!(!scroll);
```
