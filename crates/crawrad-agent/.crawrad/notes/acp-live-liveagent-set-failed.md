---
id: acp-live-liveagent-set-failed
kind: doc
target_file: src/acp_live.rs
line_hint: 215
author: 
created: 
title: "LiveAgent::set_failed"
tags: []
signature: "pub fn set_failed (& mut self , reason : String)"
---

Sets the status to Failed with reason and drops the client.

# Examples

```rust
use crawrad_agent::{LiveAgent, LiveStatus};

let mut agent = LiveAgent::new();
agent.set_failed("connection refused".to_string());
assert_eq!(agent.status(), &LiveStatus::Failed("connection refused".to_string()));
```
