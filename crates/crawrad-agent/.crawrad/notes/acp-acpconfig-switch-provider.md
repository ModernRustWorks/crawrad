---
id: acp-acpconfig-switch-provider
kind: doc
target_file: src/acp.rs
line_hint: 414
author: 
created: 
title: "AcpConfig::switch_provider"
tags: []
signature: "pub fn switch_provider (& mut self , new_provider : & str)"
---

Switches the active provider, persisting and restoring previous model selection.

# Examples

```rust
use crawrad_agent::AcpConfig;

let mut config = AcpConfig::default();
config.switch_provider("Gemini (Antigravity)");
assert_eq!(config.agent_id, "gemini");
```
