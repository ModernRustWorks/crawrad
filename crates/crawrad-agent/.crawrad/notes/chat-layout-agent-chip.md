---
id: chat-layout-agent-chip
kind: doc
target_file: src/chat_layout.rs
line_hint: 462
author: 
created: 
title: "agent_chip"
tags: []
signature: "pub fn agent_chip (messages : & [crate :: session :: ChatMessage] , agent : & str , live : & crate :: acp_live :: LiveStatus ,) -> AgentChip"
---

Computes the agent status chip for the conversation header.

The chip, from the conversation's last answer or live status: `Claude · ACP · session 3f2a9c01`,
`Claude · CLI (ACP unavailable)`, `Claude · error`, or `Claude · waiting for the first answer`.

# Examples

```rust
use crawrad_agent::chat_layout::agent_chip;
use crawrad_agent::LiveStatus;

let chip = agent_chip(&[], "Claude", &LiveStatus::Idle);
assert!(chip.text.contains("Claude"));
```
