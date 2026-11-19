---
id: chat-layout-user-bubble-indent
kind: doc
target_file: src/chat_layout.rs
line_hint: 7
author: 
created: 
title: "user_bubble_indent"
tags: []
signature: "pub fn user_bubble_indent (text : & str , meta : & str , available_width : f64) -> f64"
---

Calculates the left indent that right-aligns the user's bubble.

The left indent that right-aligns the user's bubble at its own size: short
text makes a compact one-line pill, long text wraps at the minimum indent.

The widths are MEASURED on the shell's chat font (`CRAWRAD_FONTE_CORPO`,
9.75 pt): text ~6.1 px a character, the smaller time ~5.5 px, the bubble's
padding 24 px, the gap before the time 10 px, the avatar and its spacing 36 px.

# Examples

```rust
use crawrad_agent::chat_layout::user_bubble_indent;

let indent = user_bubble_indent("hello", "10:00", 400.0);
assert!(indent >= 24.0);
```
