---
id: session-queuedmessage
kind: doc
target_file: src/session.rs
line_hint: 484
author: 
created: 
title: "QueuedMessage"
tags: []
signature: "pub struct QueuedMessage"
---

A queued message waiting to be sent to the AI assistant.

# Examples

```rust
use crawrad_agent::QueuedMessage;

let qm = QueuedMessage::new("msg_1", "Hello assistant");
assert_eq!(qm.text, "Hello assistant");
```
