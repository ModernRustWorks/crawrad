---
id: targets-makepad-control-events-pointer-events
kind: doc
target_file: src/targets/makepad/control_events.rs
line_hint: 27
author: 
created: 
title: "POINTER_EVENTS"
tags: []
signature: "pub const POINTER_EVENTS : [(& str , & str) ; 7] = [(\"on_mouse_down\" , \"Down\") , (\"on_mouse_activate\" , \"Activate\") , (\"on_mouse_up\" , \"Up\") , (\"on_mouse_move\" , \"Move\") , (\"on_mouse_enter\" , \"Enter\") , (\"on_mouse_leave\" , \"Leave\") , (\"on_mouse_wheel\" , \"Wheel\") ,]"
---

The pointer events this module delivers, and the router's name for each.

# Examples

```rust
use crawrad_codegen::targets::makepad::control_events::POINTER_EVENTS;

assert!(POINTER_EVENTS.iter().any(|(name, _)| *name == "on_mouse_down"));
```
