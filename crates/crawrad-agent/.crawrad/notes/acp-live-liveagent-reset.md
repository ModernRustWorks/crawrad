---
id: acp-live-liveagent-reset
kind: doc
target_file: src/acp_live.rs
line_hint: 223
author: 
created: 
title: "LiveAgent::reset"
tags: []
signature: "pub fn reset (& mut self)"
---

Drops the live client (which terminates its subprocess) and resets to Idle.

# Examples

```rust
use crawrad_agent::{LiveAgent, LiveStatus};

let mut agent = LiveAgent::new();
agent.set_connecting("test-key".to_string());
agent.reset();
assert_eq!(agent.status(), &LiveStatus::Idle);
```
