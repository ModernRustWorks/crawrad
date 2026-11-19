---
id: chat-layout-turns-json
kind: doc
target_file: src/chat_layout.rs
line_hint: 331
author: 
created: 
title: "turns_json"
tags: []
signature: "pub fn turns_json (messages : & [crate :: session :: ChatMessage] , default_sender : & str , live : & crate :: acp_live :: LiveStatus , status : Option < & crate :: acp_status :: StatusLine > ,) -> serde_json :: Value"
---

Builds the MCP JSON representation of the chat messages.

Builds the MCP JSON representation of the chat messages; `status` is the
live status line while a turn runs (#402).

# Examples

```rust
use crawrad_agent::chat_layout::turns_json;
use crawrad_agent::LiveStatus;

let json = turns_json(&[], "Claude", &LiveStatus::Idle, None);
assert!(json.get("turns").is_some());
```
