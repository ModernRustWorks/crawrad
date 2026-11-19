---
id: acp-acpconfig-test-live-ping
kind: doc
target_file: src/acp.rs
line_hint: 676
author: 
created: 
title: "AcpConfig::test_live_ping"
tags: []
signature: "pub fn test_live_ping (& self) -> Result < String , String >"
---

Runs an explicit live LLM query ping measuring latency.

Runs an explicit live LLM query ping measuring real round-trip network and auth latency.

WARNING: This operation consumes 1 prompt quota/token.

# Errors

Returns an error if the backend is unsupported or ping execution fails.

# Examples

```rust,no_run
// Connects to network / live LLM query.
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
let ping = config.test_live_ping().unwrap();
assert!(!ping.is_empty());
```
