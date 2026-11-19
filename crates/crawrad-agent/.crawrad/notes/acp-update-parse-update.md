---
id: acp-update-parse-update
kind: doc
target_file: src/acp_update.rs
line_hint: 10
author: 
created: 
title: "parse_update"
tags: []
signature: "pub fn parse_update (update : & serde_json :: Value , titles : & mut HashMap < String , String > ,) -> Option < AcpStreamEvent >"
---

Parses a single ACP `session/update` payload into an [`AcpStreamEvent`].

Returns `None` for updates that do not map to stream events (e.g. usage, commands,
unrecognized updates, or tool updates without title change or status change).

# Examples

```rust
use crawrad_agent::AcpStreamEvent;
use crawrad_agent::acp_update::parse_update;
use std::collections::HashMap;

let payload = serde_json::json!({
    "sessionUpdate": "agent_message_chunk",
    "content": {
        "type": "text",
        "text": "hello"
    }
});
let mut titles = HashMap::new();
let event = parse_update(&payload, &mut titles);
assert_eq!(event, Some(AcpStreamEvent::TextDelta("hello".to_string())));
```
