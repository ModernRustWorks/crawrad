---
id: file-write-agentwrites
kind: doc
target_file: src/file_write.rs
line_hint: 70
author: 
created: 
title: "AgentWrites"
tags: []
signature: "pub struct AgentWrites"
---

The agent's file writes: arriving from the turn, or held for review.

Writes come in through the channel `open` makes; the ones waiting on the
user's accept or reject stay held until `decide` answers their agent.

# Examples

```rust
use crawrad_agent::{AgentWrites, FileWriteRequest};
use std::path::Path;
use std::sync::{Arc, Mutex};

let writes = AgentWrites::default();
assert!(writes.drain().is_empty());
```
