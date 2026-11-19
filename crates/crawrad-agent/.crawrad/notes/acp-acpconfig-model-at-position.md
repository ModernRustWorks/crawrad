---
id: acp-acpconfig-model-at-position
kind: doc
target_file: src/acp.rs
line_hint: 401
author: 
created: 
title: "AcpConfig::model_at_position"
tags: []
signature: "pub fn model_at_position (& self , i : usize) -> Option < String >"
---

Returns the model identifier at the given index for the active provider.

# Examples

```rust
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
assert_eq!(config.model_at_position(0), Some("auto".to_string()));
```
