---
id: agent-profile-find-profile-by-binary
kind: doc
target_file: src/agent_profile.rs
line_hint: 408
author: 
created: 
title: "find_profile_by_binary"
tags: []
signature: "pub fn find_profile_by_binary (binary : & str) -> Option < & 'static AgentProfile >"
---

Looks up an agent profile by its executable binary name.

Looks up an agent profile by its executable binary name (e.g. "agy", "claude", "codex").

# Examples

```rust
use crawrad_agent::agent_profile::find_profile_by_binary;

let p = find_profile_by_binary("claude");
assert!(p.is_some());
```
