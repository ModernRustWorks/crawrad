---
id: chat-layout-agentchip
kind: doc
target_file: src/chat_layout.rs
line_hint: 455
author: 
created: 
title: "AgentChip"
tags: []
signature: "pub struct AgentChip"
---

The chip at the top of the conversation (#402): who answers, and how.

# Examples

```rust
use crawrad_agent::chat_layout::{AgentChip, ChipLink};

let chip = AgentChip {
    text: "Claude".to_string(),
    link: ChipLink::Acp,
};
assert_eq!(chip.link, ChipLink::Acp);
```
