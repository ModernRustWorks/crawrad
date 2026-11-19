---
id: acp-acpconfig
kind: doc
target_file: src/acp.rs
line_hint: 188
author: 
created: 
title: "AcpConfig"
tags: []
signature: "pub struct AcpConfig"
---

Agent Client Protocol (ACP) configuration for AI assistants.

# Examples

```rust
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
assert_eq!(config.agent_id, "claude");
```
