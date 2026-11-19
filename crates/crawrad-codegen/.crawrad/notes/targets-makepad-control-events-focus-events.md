---
id: targets-makepad-control-events-focus-events
kind: doc
target_file: src/targets/makepad/control_events.rs
line_hint: 46
author: 
created: 
title: "FOCUS_EVENTS"
tags: []
signature: "pub const FOCUS_EVENTS : [(& str , & str) ; 5] = [(\"on_enter\" , \"Enter\") , (\"on_exit\" , \"Exit\") , (\"on_key_down\" , \"KeyDown\") , (\"on_key_up\" , \"KeyUp\") , (\"on_key_press\" , \"Char\") ,]"
---

The focus and keyboard events, and the focus router's name for each.

# Examples

```rust
use crawrad_codegen::targets::makepad::control_events::FOCUS_EVENTS;

assert!(FOCUS_EVENTS.iter().any(|(name, _)| *name == "on_enter"));
```
