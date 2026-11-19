---
id: targets-egui-form-events-formevents-empty
kind: doc
target_file: src/targets/egui/form_events.rs
line_hint: 69
author: 
created: 
title: "FormEvents::empty"
tags: []
signature: "pub fn empty (& self) -> bool"
---

Checks whether the form dispatches any events.

Does the form dispatch anything at all?

# Examples

```rust
use crawrad_codegen::targets::egui::form_events::FormEvents;

let fe = FormEvents::default();
assert!(fe.empty());
```
