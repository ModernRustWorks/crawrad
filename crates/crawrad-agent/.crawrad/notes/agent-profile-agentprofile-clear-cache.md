---
id: agent-profile-agentprofile-clear-cache
kind: doc
target_file: src/agent_profile.rs
line_hint: 195
author: 
created: 
title: "AgentProfile::clear_cache"
tags: []
signature: "pub fn clear_cache ()"
---

Resets the model cache and in-flight tracking (used by tests).

# Examples

```rust
use crawrad_agent::AgentProfile;

AgentProfile::clear_cache();
```
