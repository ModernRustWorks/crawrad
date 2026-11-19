---
id: agent-profile-agentprofile
kind: doc
target_file: src/agent_profile.rs
line_hint: 87
author: 
created: 
title: "AgentProfile"
tags: []
signature: "pub struct AgentProfile"
---

Canonical profile of an AI agent executable.

# Examples

```rust
use crawrad_agent::AgentProfile;
use crawrad_agent::PROFILES;

let profile = &PROFILES[0];
assert_eq!(profile.id, "claude");
```
