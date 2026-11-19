---
id: acp-live-liveagent-can-resume
kind: doc
target_file: src/acp_live.rs
line_hint: 275
author: 
created: 
title: "LiveAgent::can_resume"
tags: []
signature: "pub fn can_resume (& self) -> bool"
---

Whether the live agent can resume past sessions (it declared `loadSession`).

# Examples

```rust
use crawrad_agent::LiveAgent;

let agent = LiveAgent::new();
assert!(!agent.can_resume());
```
