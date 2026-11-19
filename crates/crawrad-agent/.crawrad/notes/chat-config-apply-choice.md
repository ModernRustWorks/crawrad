---
id: chat-config-apply-choice
kind: doc
target_file: src/chat_config.rs
line_hint: 166
author: 
created: 
title: "apply_choice"
tags: []
signature: "pub fn apply_choice (live : & mut crate :: acp_live :: LiveAgent , config : & mut crate :: acp :: AcpConfig , config_id : & str , value : & str ,) -> Result < Value , String >"
---

Applies a configuration option choice on the live agent.

Applies a configuration option choice on the live agent and records it in `AcpConfig`.

# Errors

Returns an error if the live agent is not ready or if the config option or value is unknown.

# Examples

```rust
use crawrad_agent::chat_config::apply_choice;
use crawrad_agent::{AcpConfig, LiveAgent};

let mut live = LiveAgent::new();
let mut cfg = AcpConfig::default();
assert!(apply_choice(&mut live, &mut cfg, "opt", "val").is_err());
```
