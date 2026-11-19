---
id: agent-profile-build-agent-args
kind: doc
target_file: src/agent_profile.rs
line_hint: 478
author: 
created: 
title: "build_agent_args"
tags: []
signature: "pub fn build_agent_args (profile : & AgentProfile , model_id : Option < & str > , session_id : Option < & str > , is_resume : bool ,) -> Vec < String >"
---

Constructs the CLI arguments for an agent invocation according to its profile.

Ensures the exact model identifier is forwarded to `--model` without collapsing.

# Examples

```rust
use crawrad_agent::agent_profile::build_agent_args;
use crawrad_agent::resolve_profile;

let profile = resolve_profile("claude");
let args = build_agent_args(profile, Some("claude-3-5-sonnet"), None, false);
assert!(args.contains(&"--model".to_string()));
```
