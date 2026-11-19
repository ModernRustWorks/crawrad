---
id: targets-egui-form-events-conversion-items
kind: doc
target_file: src/targets/egui/form_events.rs
line_hint: 538
author: 
created: 
title: "conversion_items"
tags: []
signature: "pub fn conversion_items (form : & FormEvents , button : bool , modifiers : bool , key : bool) -> String"
---

Conversion functions from egui vocabulary to ours.

The conversions from egui's vocabulary to ours that someone else asks for
(`button`, `modifiers`, `key`), less the ones already in `form.items`: two
copies of one `fn` do not compile.

# Examples

```rust
use crawrad_codegen::targets::egui::form_events::{conversion_items, FormEvents};

let form = FormEvents::default();
let items = conversion_items(&form, true, false, false);
assert!(items.contains("crawrad_button"));
```
