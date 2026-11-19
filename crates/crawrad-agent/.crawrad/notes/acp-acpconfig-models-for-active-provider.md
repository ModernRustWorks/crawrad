---
id: acp-acpconfig-models-for-active-provider
kind: doc
target_file: src/acp.rs
line_hint: 396
author: 
created: 
title: "AcpConfig::models_for_active_provider"
tags: []
signature: "pub fn models_for_active_provider (& self) -> Vec < AcpModelInfo >"
---

Returns the models applicable for the currently active provider.

# Examples

```rust
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
let models = config.models_for_active_provider();
assert!(!models.is_empty());
```
