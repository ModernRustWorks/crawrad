---
id: targets-egui-form-events-control-key-items
kind: doc
target_file: src/targets/egui/form_events.rs
line_hint: 533
author: 
created: 
title: "control_key_items"
tags: []
signature: "pub fn control_key_items (form : & FormEvents) -> String"
---

Conversion functions needed for control key events.

The conversions a control's own key events need (`control_keys`), less
the ones the form's events already put in `items`.

# Examples

```rust
use crawrad_codegen::targets::egui::form_events::{control_key_items, FormEvents};

let form = FormEvents::default();
let items = control_key_items(&form);
assert!(items.contains("crawrad_key"));
```
