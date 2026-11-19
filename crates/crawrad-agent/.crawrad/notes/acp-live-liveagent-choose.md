---
id: acp-live-liveagent-choose
kind: doc
target_file: src/acp_live.rs
line_hint: 180
author: 
created: 
title: "LiveAgent::choose"
tags: []
signature: "pub fn choose (& mut self , config_id : & str , value : & str)"
---

Applies a configuration option choice asynchronously on the live agent.

# Examples

```rust
use crawrad_agent::LiveAgent;

let mut agent = LiveAgent::new();
agent.choose("model", "sonnet");
assert_eq!(agent.last_error(), None);
```
