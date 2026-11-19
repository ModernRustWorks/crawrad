---
id: events-changing
kind: doc
target_file: src/lib.rs
line_hint: 355
author: 
created: 
title: "Events::CHANGING"
tags: []
signature: "pub const CHANGING : EventDesc = EventDesc :: new (\"on_changing\" , \"OnChanging\" , \"fn(&mut self, allow_change: &mut bool)\" ,)"
---

Before the change: setting `allow_change` to false keeps what was there.

# Examples

```rust
use crawrad_registry::Events;

assert_eq!(Events::CHANGING.name, "on_changing");
```
