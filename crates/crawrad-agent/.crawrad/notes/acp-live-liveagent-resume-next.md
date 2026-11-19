---
id: acp-live-liveagent-resume-next
kind: doc
target_file: src/acp_live.rs
line_hint: 262
author: 
created: 
title: "LiveAgent::resume_next"
tags: []
signature: "pub fn resume_next (& mut self , session : Option < String >)"
---

Sets the ACP session ID to resume on next warm-up.

The next warm-up resumes ACP session `session` (#478: a conversation
reopened from the history); `reset` keeps it.

# Examples

```rust
use crawrad_agent::LiveAgent;

let mut agent = LiveAgent::new();
agent.resume_next(Some("session-123".to_string()));
```
