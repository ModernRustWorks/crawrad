---
id: agent-profile-profile-for
kind: doc
target_file: src/agent_profile.rs
line_hint: 423
author: 
created: 
title: "profile_for"
tags: []
signature: "pub fn profile_for (agent_id : & str , provider : & str) -> & 'static AgentProfile"
---

The profile of the configured agent.

The profile of the configured agent: by its id, else resolved from its provider.

# Examples

```rust
use crawrad_agent::agent_profile::profile_for;

let p = profile_for("claude", "Claude Agent");
assert_eq!(p.id, "claude");
```
