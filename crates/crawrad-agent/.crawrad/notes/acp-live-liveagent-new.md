---
id: acp-live-liveagent-new
kind: doc
target_file: src/acp_live.rs
line_hint: 71
author: 
created: 
title: "LiveAgent::new"
tags: []
signature: "pub fn new () -> Self"
---

Creates a new idle live agent.

# Examples

```rust
use crawrad_agent::LiveAgent;

let agent = LiveAgent::new();
assert_eq!(agent.agent_key(), "");
```
