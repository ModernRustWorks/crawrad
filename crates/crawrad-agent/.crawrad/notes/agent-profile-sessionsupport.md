---
id: agent-profile-sessionsupport
kind: doc
target_file: src/agent_profile.rs
line_hint: 19
author: 
created: 
title: "SessionSupport"
tags: []
signature: "pub enum SessionSupport"
---

Multi-turn session continuity strategy supported by the agent executable.

# Examples

```rust
use crawrad_agent::SessionSupport;

let s = SessionSupport::Pinned;
assert_eq!(s, SessionSupport::Pinned);
```
