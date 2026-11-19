---
id: acp-registry-agentcard
kind: doc
target_file: src/acp_registry.rs
line_hint: 754
author: 
created: 
title: "AgentCard"
tags: []
signature: "pub struct AgentCard"
---

The "Agent in use" card of the Options page, ready to show.

# Examples

```rust
use crawrad_agent::acp_registry::{AgentCard, AgentState, AcpAgentOperationalStatus};

let card = AgentCard {
    status: AcpAgentOperationalStatus {
        active_name: "Claude".to_string(),
        active_version: "1.0".to_string(),
        command: "claude".to_string(),
        status_title: "Ready".to_string(),
        status_detail: "Detail".to_string(),
        target_url: "https://example.com".to_string(),
        is_installed: true,
    },
    state: AgentState::Ready,
    hint: "Installed".to_string(),
    detail: "Registered".to_string(),
};
assert_eq!(card.state, AgentState::Ready);
```
