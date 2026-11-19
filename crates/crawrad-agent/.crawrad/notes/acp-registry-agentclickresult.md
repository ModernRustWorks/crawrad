---
id: acp-registry-agentclickresult
kind: doc
target_file: src/acp_registry.rs
line_hint: 964
author: 
created: 
title: "AgentClickResult"
tags: []
signature: "pub enum AgentClickResult"
---

Result of clicking an agent action in the options catalog.

# Examples

```rust
use crawrad_agent::acp_registry::AgentClickResult;

let res = AgentClickResult::Ignored;
assert!(matches!(res, AgentClickResult::Ignored));
```
