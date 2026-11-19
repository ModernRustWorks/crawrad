---
id: agent-profile-find-profile-by-id
kind: doc
target_file: src/agent_profile.rs
line_hint: 396
author: 
created: 
title: "find_profile_by_id"
tags: []
signature: "pub fn find_profile_by_id (id : & str) -> Option < & 'static AgentProfile >"
---

Looks up an agent profile by its stable canonical identifier.

# Examples

```rust
use crawrad_agent::find_profile_by_id;

let p = find_profile_by_id("claude");
assert!(p.is_some());
```
