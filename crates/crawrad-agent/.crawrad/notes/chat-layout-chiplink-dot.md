---
id: chat-layout-chiplink-dot
kind: doc
target_file: src/chat_layout.rs
line_hint: 436
author: 
created: 
title: "ChipLink::dot"
tags: []
signature: "pub fn dot (self) -> & 'static str"
---

The chip's dot that shows this state, one of [`CHIP_DOTS`].

# Examples

```rust
use crawrad_agent::chat_layout::ChipLink;

assert_eq!(ChipLink::Acp.dot(), "chip_dot_acp");
```
