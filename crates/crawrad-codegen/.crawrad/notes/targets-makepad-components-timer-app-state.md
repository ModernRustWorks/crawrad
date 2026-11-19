---
id: targets-makepad-components-timer-app-state
kind: doc
target_file: src/targets/makepad/components/timer.rs
line_hint: 93
author: 
created: 
title: "app_state"
tags: []
signature: "pub fn app_state (node : & Node , field : & str) -> Result < AppState , crate :: Erro >"
---

Emits Makepad `App` state for a timer component.

The `Ticker` in the `App`, and on its tick the form's `OnTimer` (then the
sync, so what the handler changed is drawn). A Timer with no handler still
ticks: its `enabled` and interval are honored, nothing is called.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::timer::app_state;
use crawrad_ir::Node;

let node = Node::new("timer1", "Timer");
let state = app_state(&node, "timer1").unwrap();
assert!(state.uses.contains("Ticker"));
assert!(state.field.contains("timer1_ticker"));
```

# Errors

Returns an error if the configured timer handler is not a valid identifier.
