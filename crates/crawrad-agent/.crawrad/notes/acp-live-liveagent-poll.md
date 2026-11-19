---
id: acp-live-liveagent-poll
kind: doc
target_file: src/acp_live.rs
line_hint: 280
author: 
created: 
title: "LiveAgent::poll"
tags: []
signature: "pub fn poll (& mut self) -> bool"
---

Takes the warm-up or choice answer once arrived.

Takes the warm-up's or choice's answer once it has arrived. Returns whether anything changed.

# Examples

```rust
use crawrad_agent::LiveAgent;

let mut agent = LiveAgent::new();
assert!(!agent.poll());
```
