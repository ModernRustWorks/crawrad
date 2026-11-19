---
id: agent-profile-modelspec
kind: doc
target_file: src/agent_profile.rs
line_hint: 42
author: 
created: 
title: "ModelSpec"
tags: []
signature: "pub struct ModelSpec"
---

Metadata description of a supported language model.

# Examples

```rust
use crawrad_agent::ModelSpec;

let spec = ModelSpec {
    id: "sonnet".to_string(),
    display_name: "Sonnet".to_string(),
    description: "Standard".to_string(),
    supports_reasoning: true,
};
assert_eq!(spec.id, "sonnet");
```
