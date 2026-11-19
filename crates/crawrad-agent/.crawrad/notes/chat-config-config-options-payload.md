---
id: chat-config-config-options-payload
kind: doc
target_file: src/chat_config.rs
line_hint: 151
author: 
created: 
title: "config_options_payload"
tags: []
signature: "pub fn config_options_payload (live : & crate :: acp_live :: LiveAgent) -> Value"
---

Builds the MCP JSON response for `chat.get_config_options`.

# Examples

```rust
use crawrad_agent::LiveAgent;
use crawrad_agent::chat_config::config_options_payload;

let live = LiveAgent::new();
let payload = config_options_payload(&live);
assert!(payload.get("live").is_some());
```
