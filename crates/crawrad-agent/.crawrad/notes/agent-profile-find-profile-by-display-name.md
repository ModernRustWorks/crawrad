---
id: agent-profile-find-profile-by-display-name
kind: doc
target_file: src/agent_profile.rs
line_hint: 401
author: 
created: 
title: "find_profile_by_display_name"
tags: []
signature: "pub fn find_profile_by_display_name (name : & str) -> Option < & 'static AgentProfile >"
---

Looks up an agent profile by its UI display label.

# Examples

```rust
use crawrad_agent::find_profile_by_display_name;

let p = find_profile_by_display_name("Claude Agent");
assert!(p.is_some());
```
