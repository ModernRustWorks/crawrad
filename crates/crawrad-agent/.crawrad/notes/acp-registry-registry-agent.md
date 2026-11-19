---
id: acp-registry-registry-agent
kind: doc
target_file: src/acp_registry.rs
line_hint: 847
author: 
created: 
title: "registry_agent"
tags: []
signature: "pub fn registry_agent (id : & str) -> Option < AcpAgentEntry >"
---

Looks up an agent entry in the cached registry by ID.

# Examples

```rust
use crawrad_agent::registry_agent;

let agent = registry_agent("nonexistent_id");
assert!(agent.is_none());
```
