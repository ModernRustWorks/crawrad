---
id: acp-live-liveagent-session-id
kind: doc
target_file: src/acp_live.rs
line_hint: 267
author: 
created: 
title: "LiveAgent::session_id"
tags: []
signature: "pub fn session_id (& self) -> Option < & str >"
---

The live session's id, once ready.

# Examples

```rust
use crawrad_agent::LiveAgent;

let agent = LiveAgent::new();
assert_eq!(agent.session_id(), None);
```
