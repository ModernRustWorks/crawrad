---
id: debug-debugevent
kind: doc
target_file: src/debug.rs
line_hint: 30
author: 
created: 
title: "DebugEvent"
tags: []
signature: "pub enum DebugEvent"
---

What changed in one [`Debug::pump`], for the screen.

# Examples

```rust
use crawrad_host::DebugEvent;

let event = DebugEvent::Resumed;
assert_eq!(event, DebugEvent::Resumed);
```
