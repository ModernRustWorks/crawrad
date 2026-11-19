---
id: acp-acpconfig-provider-index
kind: doc
target_file: src/acp.rs
line_hint: 276
author: 
created: 
title: "AcpConfig::provider_index"
tags: []
signature: "pub fn provider_index (& self) -> Option < usize >"
---

Returns the active provider index in PROVIDERS.

# Examples

```rust
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
assert_eq!(config.provider_index(), Some(0));
```
