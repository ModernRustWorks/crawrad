---
id: client-acpstreamevent
kind: doc
target_file: src/client.rs
line_hint: 22
author: 
created: 
title: "AcpStreamEvent"
tags: []
signature: "pub enum AcpStreamEvent"
---

Events streamed from an ACP agent during prompt execution.

# Examples

```rust
use crawrad_agent::AcpStreamEvent;

let ev = AcpStreamEvent::TextDelta("hello".to_string());
assert!(matches!(ev, AcpStreamEvent::TextDelta(_)));
```
