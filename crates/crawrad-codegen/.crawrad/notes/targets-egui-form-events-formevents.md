---
id: targets-egui-form-events-formevents
kind: doc
target_file: src/targets/egui/form_events.rs
line_hint: 41
author: 
created: 
title: "FormEvents"
tags: []
signature: "pub struct FormEvents"
---

The pieces the form generator has to place, each in its own spot.

# Examples

```rust
use crawrad_codegen::targets::egui::form_events::FormEvents;

let fe = FormEvents::default();
assert!(fe.empty());
```
