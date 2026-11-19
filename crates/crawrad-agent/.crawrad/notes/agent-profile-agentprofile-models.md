---
id: agent-profile-agentprofile-models
kind: doc
target_file: src/agent_profile.rs
line_hint: 128
author: 
created: 
title: "AgentProfile::models"
tags: []
signature: "pub fn models (& self) -> Vec < ModelSpec >"
---

Returns the available models for this profile.

This method is strictly non-blocking and safe for UI rendering loops:
- If `Static`, returns the static specifications.
- If `AgentCommand`, returns the cached models if available. If the cache is empty,
  it schedules a 1x background fetch thread and returns the verified static fallback.

# Examples

```rust
use crawrad_agent::resolve_profile;

let profile = resolve_profile("claude");
let models = profile.models();
assert!(!models.is_empty());
```
