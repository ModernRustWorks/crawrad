---
id: edit-ask-agenteditasks
kind: doc
target_file: src/edit_ask.rs
line_hint: 103
author: 
created: 
title: "AgentEditAsks"
tags: []
signature: "pub struct AgentEditAsks"
---

Edit permission requests waiting on their diff review.

One per file; answered when the user accepts or rejects the diff.

# Examples

```rust
use crawrad_agent::AgentEditAsks;

let mut asks = AgentEditAsks::default();
asks.decide(std::path::Path::new("a.rs"), true);
```
