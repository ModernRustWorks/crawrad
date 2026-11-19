---
id: acp-registry-agent-state
kind: doc
target_file: src/acp_registry.rs
line_hint: 741
author: 
created: 
title: "agent_state"
tags: []
signature: "pub fn agent_state (installed : bool , connected : bool , last_error : Option < & str >) -> AgentState"
---

Computes agent state from installation and connection status.

`connected`: the live chat session is up. `last_error`: the last connection message, if any.

# Examples

```rust
use crawrad_agent::acp_registry::{AgentState, agent_state};

assert_eq!(agent_state(true, true, None), AgentState::Ready);
```
