---
id: acp-live-liveagent-client
kind: doc
target_file: src/acp_live.rs
line_hint: 153
author: 
created: 
title: "LiveAgent::client"
tags: []
signature: "pub fn client (& self) -> Option < Arc < AcpClient > >"
---

Returns an Arc clone of the live ACP client if Ready, or None otherwise.

# Examples

```rust
use crawrad_agent::LiveAgent;

let agent = LiveAgent::new();
assert!(agent.client().is_none());
```
