---
id: agent-profile-agentprofile-set-background-fetch
kind: doc
target_file: src/agent_profile.rs
line_hint: 123
author: 
created: 
title: "AgentProfile::set_background_fetch"
tags: []
signature: "pub fn set_background_fetch (enabled : bool)"
---

Enables or disables background model fetching from external agent CLIs.

Tests turn it off so no real agent CLI runs and the cache only changes when a test changes it.

# Examples

```rust
use crawrad_agent::AgentProfile;

AgentProfile::set_background_fetch(false);
```
