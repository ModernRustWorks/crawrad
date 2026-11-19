---
id: chat-layout-chiplink-as-str
kind: doc
target_file: src/chat_layout.rs
line_hint: 426
author: 
created: 
title: "ChipLink::as_str"
tags: []
signature: "pub fn as_str (self) -> & 'static str"
---

The word `view.get_chat` reports.

# Examples

```rust
use crawrad_agent::chat_layout::ChipLink;

assert_eq!(ChipLink::Acp.as_str(), "acp");
```
