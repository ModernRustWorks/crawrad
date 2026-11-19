---
id: events-focusrouter-route
kind: doc
target_file: src/events.rs
line_hint: 284
author: 
created: 
title: "FocusRouter::route"
tags: []
signature: "pub fn route (& mut self , event : & Event , pointer : & [PointerHit] , focusable : & [usize] , key_focus : Option < usize > ,) -> Vec < FocusHit >"
---

Computes focus and keyboard events from input events and pointer hits.

The focus and keyboard events `event` means, given the pointer hits of
the same event, the controls that can take the focus, and the one
whose widget has the keyboard focus now (`key_focus`).

# Examples

```rust
use makepad_widgets::Event;
use crawrad_runtime_makepad::events::FocusRouter;

let mut router = FocusRouter::default();
let hits = router.route(&Event::Pause, &[], &[0, 1], None);
assert!(hits.is_empty());
```
