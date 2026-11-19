---
id: acp-live-liveagent-key-of
kind: doc
target_file: src/acp_live.rs
line_hint: 76
author: 
created: 
title: "LiveAgent::key_of"
tags: []
signature: "pub fn key_of (config : & AcpConfig) -> String"
---

Computes the key identifying the agent configuration.

Computes the key identifying the agent configuration (agent_id + endpoint + endpoint_args).

# Examples

```rust
use crawrad_agent::{AcpConfig, LiveAgent};

let config = AcpConfig::default();
let key = LiveAgent::key_of(&config);
assert!(key.starts_with("claude:"));
```
