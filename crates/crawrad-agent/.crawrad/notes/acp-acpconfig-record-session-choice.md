---
id: acp-acpconfig-record-session-choice
kind: doc
target_file: src/acp.rs
line_hint: 255
author: 
created: 
title: "AcpConfig::record_session_choice"
tags: []
signature: "pub fn record_session_choice (& mut self , config_id : & str , value : & str)"
---

Records a session choice (agent_id -> config_id -> value).

# Examples

```rust
use crawrad_agent::AcpConfig;

let mut config = AcpConfig::default();
config.record_session_choice("mode", "code");
assert_eq!(config.session_choices.get("claude").unwrap().get("mode").unwrap(), "code");
```
