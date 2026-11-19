---
id: agent-profile-agentprofile-refresh-models-in-background
kind: doc
target_file: src/agent_profile.rs
line_hint: 171
author: 
created: 
title: "AgentProfile::refresh_models_in_background"
tags: []
signature: "pub fn refresh_models_in_background (& self)"
---

Forces an asynchronous refresh of the model cache in background.

Forces an asynchronous refresh of the model cache from the agent CLI in a background thread.

# Examples

```rust,no_run
// Spawns background CLI model query thread.
use crawrad_agent::resolve_profile;

let profile = resolve_profile("gemini");
profile.refresh_models_in_background();
```
