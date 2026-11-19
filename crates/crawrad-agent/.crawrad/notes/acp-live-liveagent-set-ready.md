---
id: acp-live-liveagent-set-ready
kind: doc
target_file: src/acp_live.rs
line_hint: 208
author: 
created: 
title: "LiveAgent::set_ready"
tags: []
signature: "pub fn set_ready (& mut self , client : Arc < AcpClient > , session : String)"
---

Sets the status to Ready with the established client and session.

# Examples

```rust,no_run
// Requires spawned ACP client process.
use crawrad_agent::{AcpClient, LiveAgent, LiveStatus};
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(AcpClient::spawn(Path::new("dummy"), &[], None, false).unwrap());
let mut agent = LiveAgent::new();
agent.set_ready(client, "session-1".to_string());
assert!(matches!(agent.status(), LiveStatus::Ready { .. }));
```
