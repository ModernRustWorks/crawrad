---
id: protocol-event
kind: doc
target_file: src/protocol.rs
line_hint: 44
author: 
created: 
title: "Event"
tags: []
signature: "pub struct Event"
---

An event notification from the adapter.

Something that happened in the adapter: `stopped`, `output`, `terminated`…

# Examples

```rust
use serde_json::json;
use crawrad_dap::Event;

let ev = Event {
    seq: 1,
    event: "terminated".into(),
    body: json!({}),
};
assert_eq!(ev.event, "terminated");
```
