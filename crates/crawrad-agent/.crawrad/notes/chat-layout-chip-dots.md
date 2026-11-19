---
id: chat-layout-chip-dots
kind: doc
target_file: src/chat_layout.rs
line_hint: 447
author: 
created: 
title: "CHIP_DOTS"
tags: []
signature: "pub const CHIP_DOTS : [& str ; 4] = [\"chip_dot_waiting\" , \"chip_dot_acp\" , \"chip_dot_cli\" , \"chip_dot_failed\" ,]"
---

The chip's four dots, by their DSL ids: exactly one is shown.

# Examples

```rust
use crawrad_agent::chat_layout::CHIP_DOTS;

assert_eq!(CHIP_DOTS.len(), 4);
```
