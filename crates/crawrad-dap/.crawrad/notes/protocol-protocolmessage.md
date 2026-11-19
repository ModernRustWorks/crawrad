---
id: protocol-protocolmessage
kind: doc
target_file: src/protocol.rs
line_hint: 13
author: 
created: 
title: "ProtocolMessage"
tags: []
signature: "pub enum ProtocolMessage"
---

Every message on the wire: the `type` field says which.

# Examples

```rust
use serde_json::json;
use crawrad_dap::{Event, ProtocolMessage};

let msg = ProtocolMessage::Event(Event {
    seq: 1,
    event: "initialized".into(),
    body: json!({}),
});
assert!(matches!(msg, ProtocolMessage::Event(_)));
```
