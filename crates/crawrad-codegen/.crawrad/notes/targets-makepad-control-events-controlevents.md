---
id: targets-makepad-control-events-controlevents
kind: doc
target_file: src/targets/makepad/control_events.rs
line_hint: 58
author: 
created: 
title: "ControlEvents"
tags: []
signature: "pub struct ControlEvents"
---

Code and state added to `App` for control event routing.

What the controls' events add to the `App`, and which conversions they
need; `None` when no control has a handler for any of them.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::codegen::AppState;
use crawrad_codegen::targets::makepad::control_events::ControlEvents;

let events = ControlEvents {
    app: AppState::default(),
    needs_button: false,
    needs_modifiers: false,
    needs_key: false,
    dsl: String::new(),
};
assert!(!events.needs_button);
```
