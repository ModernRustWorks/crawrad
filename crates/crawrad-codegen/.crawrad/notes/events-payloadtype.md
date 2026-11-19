---
id: events-payloadtype
kind: doc
target_file: src/events.rs
line_hint: 144
author: 
created: 
title: "PayloadType"
tags: []
signature: "pub struct PayloadType"
---

A payload type definition for event parameters.

One payload type: its name, the other payload types it holds inside, and
the Rust that defines it.

# Examples

```rust
use crawrad_codegen::events::PayloadType;

let payload = PayloadType {
    name: "CustomPayload",
    needs: &[],
    text: "pub struct CustomPayload;",
};
assert_eq!(payload.name, "CustomPayload");
```
