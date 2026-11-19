---
id: chat-config-mcp-call
kind: doc
target_file: src/chat_config.rs
line_hint: 194
author: 
created: 
title: "mcp_call"
tags: []
signature: "pub fn mcp_call (name : & str , args : & Value , live : & mut crate :: acp_live :: LiveAgent , config : & mut crate :: acp :: AcpConfig , chat : (& [crate :: session :: ChatMessage] , & str) ,) -> Result < Value , String >"
---

Handles chat MCP tool invocations.

The chat's MCP tools: `chat.get_config_options`, `chat.set_config_option`
(`{config_id, value}`, the same path as a click) and `chat.copy_message`
(the text to copy comes back as `copied`; the shell owns the clipboard).

# Errors

Returns an error if copying a message fails or setting a configuration option fails.

# Examples

```rust
use crawrad_agent::chat_config::mcp_call;
use crawrad_agent::{AcpConfig, LiveAgent};
use serde_json::json;

let mut live = LiveAgent::new();
let mut cfg = AcpConfig::default();
let res = mcp_call("chat.get_config_options", &json!({}), &mut live, &mut cfg, (&[], "")).unwrap();
assert!(res.get("live").is_some());
```
