---
id: agent-profile-staticmodelspec
kind: doc
target_file: src/agent_profile.rs
line_hint: 67
author: 
created: 
title: "StaticModelSpec"
tags: []
signature: "pub struct StaticModelSpec"
---

Static definition helper for const model tables.

# Examples

```rust
use crawrad_agent::StaticModelSpec;

let s = StaticModelSpec {
    id: "sonnet",
    display_name: "Sonnet",
    description: "Standard",
    supports_reasoning: true,
};
assert_eq!(s.id, "sonnet");
```
