---
id: agent-profile-resolve-profile
kind: doc
target_file: src/agent_profile.rs
line_hint: 415
author: 
created: 
title: "resolve_profile"
tags: []
signature: "pub fn resolve_profile (id_or_name : & str) -> & 'static AgentProfile"
---

Resolves an agent profile by ID, display name, or binary.

Resolves an agent profile by canonical ID, display name, or binary, falling back to Claude.

# Examples

```rust
use crawrad_agent::resolve_profile;

let p = resolve_profile("claude");
assert_eq!(p.id, "claude");
```
