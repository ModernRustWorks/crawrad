---
id: events-return
kind: doc
target_file: src/lib.rs
line_hint: 388
author: 
created: 
title: "Events::RETURN"
tags: []
signature: "pub const RETURN : EventDesc = EventDesc :: new (\"on_return\" , \"OnReturn\" , \"fn(&mut self)\")"
---

Enter was pressed in an edit field (#434).

# Examples

```rust
use crawrad_registry::Events;

assert_eq!(Events::RETURN.name, "on_return");
```
