---
id: acp-live-liveagent-config-options
kind: doc
target_file: src/acp_live.rs
line_hint: 162
author: 
created: 
title: "LiveAgent::config_options"
tags: []
signature: "pub fn config_options (& self) -> Vec < Value >"
---

Configuration options for the active session.

Configuration options for the active session, with any pending choice applied on top.

# Examples

```rust
use crawrad_agent::LiveAgent;

let agent = LiveAgent::new();
assert!(agent.config_options().is_empty());
```
