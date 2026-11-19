---
id: acp-acpconfig-add-custom-model-for-provider
kind: doc
target_file: src/acp.rs
line_hint: 357
author: 
created: 
title: "AcpConfig::add_custom_model_for_provider"
tags: []
signature: "pub fn add_custom_model_for_provider (& mut self , provider : & str , model_id : & str)"
---

Adds a custom model identifier for the specified provider.

# Examples

```rust
use crawrad_agent::AcpConfig;

let mut config = AcpConfig::default();
config.add_custom_model_for_provider("Claude Agent", "my-custom-model");
assert_eq!(config.model, "my-custom-model");
```
