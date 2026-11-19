---
id: agent-profile-agentprofile-trigger-background-fetch-once
kind: doc
target_file: src/agent_profile.rs
line_hint: 144
author: 
created: 
title: "AgentProfile::trigger_background_fetch_once"
tags: []
signature: "pub fn trigger_background_fetch_once (& self)"
---

Triggers a 1x background fetch of live models without blocking.

Triggers a 1x background fetch of live models from the agent CLI without blocking the caller.

# Examples

```rust,no_run
// Triggers background CLI model query thread.
use crawrad_agent::resolve_profile;

let profile = resolve_profile("gemini");
profile.trigger_background_fetch_once();
```
