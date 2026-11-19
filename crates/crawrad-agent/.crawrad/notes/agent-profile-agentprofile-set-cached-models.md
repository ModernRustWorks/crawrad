---
id: agent-profile-agentprofile-set-cached-models
kind: doc
target_file: src/agent_profile.rs
line_hint: 188
author: 
created: 
title: "AgentProfile::set_cached_models"
tags: []
signature: "pub fn set_cached_models (& self , models : Vec < ModelSpec >)"
---

Overwrites the in-memory cache directly (used by tests or pre-warming).

# Examples

```rust
use crawrad_agent::resolve_profile;

let profile = resolve_profile("claude");
profile.set_cached_models(Vec::new());
```
