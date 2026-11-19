---
id: events-moved
kind: doc
target_file: src/lib.rs
line_hint: 353
author: 
created: 
title: "Events::MOVED"
tags: []
signature: "pub const MOVED : EventDesc = EventDesc :: new (\"on_moved\" , \"OnMoved\" , \"fn(&mut self)\")"
---

A Splitter was dragged and let go (#461).

# Examples

```rust
use crawrad_registry::Events;

assert_eq!(Events::MOVED.name, "on_moved");
```
