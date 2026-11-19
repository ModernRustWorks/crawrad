---
id: acp-live-liveagent-set-connecting
kind: doc
target_file: src/acp_live.rs
line_hint: 201
author: 
created: 
title: "LiveAgent::set_connecting"
tags: []
signature: "pub fn set_connecting (& mut self , agent_key : String)"
---

Sets the status to Connecting with the target agent key.

# Examples

```rust
use crawrad_agent::{LiveAgent, LiveStatus};

let mut agent = LiveAgent::new();
agent.set_connecting("test-key".to_string());
assert_eq!(agent.status(), &LiveStatus::Connecting);
```
