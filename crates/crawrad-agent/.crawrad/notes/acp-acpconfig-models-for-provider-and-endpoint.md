---
id: acp-acpconfig-models-for-provider-and-endpoint
kind: doc
target_file: src/acp.rs
line_hint: 295
author: 
created: 
title: "AcpConfig::models_for_provider_and_endpoint"
tags: []
signature: "pub fn models_for_provider_and_endpoint (& self , provider : & str , _endpoint : & str ,) -> Vec < AcpModelInfo >"
---

Returns the models applicable for the specified provider and endpoint.

# Examples

```rust
use crawrad_agent::AcpConfig;

let config = AcpConfig::default();
let models = config.models_for_provider_and_endpoint("Claude Agent", "");
assert!(!models.is_empty());
```
