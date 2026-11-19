---
id: acp-live-liveagent-last-error
kind: doc
target_file: src/acp_live.rs
line_hint: 148
author: 
created: 
title: "LiveAgent::last_error"
tags: []
signature: "pub fn last_error (& self) -> Option < & str >"
---

Returns the last error recorded during config option selection.

# Examples

```rust
use crawrad_agent::LiveAgent;

let agent = LiveAgent::new();
assert_eq!(agent.last_error(), None);
```
