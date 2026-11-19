---
id: components-base-with-classic-events
kind: doc
target_file: src/components/base.rs
line_hint: 176
author: 
created: 
title: "with_classic_events"
tags: []
signature: "pub fn with_classic_events (mut d : ComponentDesc) -> ComponentDesc"
---

Enriches a component descriptor with its standard classic events.

`d` with the classic events of its kind that it does not declare yet, and
its events in the Object Inspector's order: by name, as classic RAD lists
them. Both targets deliver these for any class (their routers), so
declaring one is all it takes.

# Examples

```rust
use crawrad_registry::{button, with_classic_events};

let desc = with_classic_events(button());
assert!(!desc.events.is_empty());
```
