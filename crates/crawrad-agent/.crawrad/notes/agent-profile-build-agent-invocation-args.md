---
id: agent-profile-build-agent-invocation-args
kind: doc
target_file: src/agent_profile.rs
line_hint: 558
author: 
created: 
title: "build_agent_invocation_args"
tags: []
signature: "pub fn build_agent_invocation_args (profile : & AgentProfile , prompt : & str , model_id : Option < & str > , session_id : Option < & str > , is_resume : bool ,) -> (Vec < String > , String)"
---

Constructs full CLI arguments and stdin payload for an agent invocation.

Constructs full CLI arguments and stdin payload for an agent invocation with a user prompt.

# Examples

```rust
use crawrad_agent::{build_agent_invocation_args, resolve_profile};

let profile = resolve_profile("claude");
let (args, stdin) = build_agent_invocation_args(profile, "hello", None, None, false);
assert_eq!(stdin, "hello");
```
