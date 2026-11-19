---
id: chat-layout-chiplink
kind: doc
target_file: src/chat_layout.rs
line_hint: 412
author: 
created: 
title: "ChipLink"
tags: []
signature: "pub enum ChipLink"
---

The chip's state: the dot's colour follows it.

# Examples

```rust
use crawrad_agent::chat_layout::ChipLink;

assert_eq!(ChipLink::Waiting.as_str(), "waiting");
```
