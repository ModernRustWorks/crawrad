---
id: acp-registry-agent-card
kind: doc
target_file: src/acp_registry.rs
line_hint: 765
author: 
created: 
title: "agent_card"
tags: []
signature: "pub fn agent_card (cfg : & crate :: AcpConfig , status_msg : Option < & str > , connected : bool) -> AgentCard"
---

Builds the Agent in use card for the given configuration.

The card for `cfg`: `status_msg` is the last connection message, `connected`
whether the chat's live session is up.

# Examples

```rust
use crawrad_agent::{AcpConfig, acp_registry::agent_card};

let cfg = AcpConfig::default();
let card = agent_card(&cfg, None, false);
assert!(!card.hint.is_empty());
```
