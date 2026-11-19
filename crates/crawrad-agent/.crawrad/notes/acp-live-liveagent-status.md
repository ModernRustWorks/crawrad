---
id: acp-live-liveagent-status
kind: doc
target_file: src/acp_live.rs
line_hint: 138
author: 
created: 
title: "LiveAgent::status"
tags: []
signature: "pub fn status (& self) -> & LiveStatus"
---

Current lifecycle status.

# Examples

```rust
use crawrad_agent::{LiveAgent, LiveStatus};

let agent = LiveAgent::new();
assert_eq!(agent.status(), &LiveStatus::Idle);
```
