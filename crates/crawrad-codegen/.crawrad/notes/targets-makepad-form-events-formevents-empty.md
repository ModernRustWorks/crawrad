---
id: targets-makepad-form-events-formevents-empty
kind: doc
target_file: src/targets/makepad/form_events.rs
line_hint: 58
author: 
created: 
title: "FormEvents::empty"
tags: []
signature: "pub fn empty (& self) -> bool"
---

Returns `true` if the form dispatches no events.

Does the form dispatch anything from an event?

# Examples

```rust
use crawrad_codegen::targets::makepad::form_events::FormEvents;

let events = FormEvents::default();
assert!(events.empty());
```
