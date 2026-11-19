---
id: acp-acpconfig-remove-model-for-provider
kind: doc
target_file: src/acp.rs
line_hint: 375
author: 
created: 
title: "AcpConfig::remove_model_for_provider"
tags: []
signature: "pub fn remove_model_for_provider (& mut self , provider : & str , model_id : & str)"
---

Removes a model identifier for the specified provider.

# Examples

```rust
use crawrad_agent::AcpConfig;

let mut config = AcpConfig::default();
config.add_custom_model_for_provider("Claude Agent", "my-custom-model");
config.remove_model_for_provider("Claude Agent", "my-custom-model");
assert_ne!(config.model, "my-custom-model");
```
