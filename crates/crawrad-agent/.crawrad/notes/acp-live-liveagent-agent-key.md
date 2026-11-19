---
id: acp-live-liveagent-agent-key
kind: doc
target_file: src/acp_live.rs
line_hint: 143
author: 
created: 
title: "LiveAgent::agent_key"
tags: []
signature: "pub fn agent_key (& self) -> & str"
---

The key of the currently associated agent.

# Examples

```rust
use crawrad_agent::LiveAgent;

let agent = LiveAgent::new();
assert_eq!(agent.agent_key(), "");
```
