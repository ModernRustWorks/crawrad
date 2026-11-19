---
id: acp-registry-agentstate
kind: doc
target_file: src/acp_registry.rs
line_hint: 701
author: 
created: 
title: "AgentState"
tags: []
signature: "pub enum AgentState"
---

Operational state and fix action for an agent in use.

What the Options page says about the agent in use, and the one action that fixes it.

# Examples

```rust
use crawrad_agent::acp_registry::AgentState;

assert_eq!(AgentState::Ready.label(), "Ready");
```
