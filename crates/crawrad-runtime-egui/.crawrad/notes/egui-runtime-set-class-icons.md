---
id: egui-runtime-set-class-icons
kind: doc
target_file: src/lib.rs
line_hint: 590
author: 
created: 
title: "EguiRuntime::set_class_icons"
tags: []
signature: "pub fn set_class_icons (& mut self , icons : BTreeMap < String , Vec < crawrad_ir :: icon :: IconStep > >)"
---

Hands in each class's palette icon, so non-visual badges wear it.

The shell owns the catalogue and passes the steps in; this adapter never
imports it. A class missing from the map keeps the initials in its badge,
the same rule as the Makepad canvas.

# Examples

```rust
use std::collections::BTreeMap;
use crawrad_ir::icon::IconStep;
use crawrad_runtime_egui::EguiRuntime;

let mut rt = EguiRuntime::new(egui::Context::default());
let mut icons = BTreeMap::new();
icons.insert("Timer".to_string(), vec![IconStep::Circle { x: 8.0, y: 8.0, r: 6.0 }, IconStep::Stroke]);
rt.set_class_icons(icons);
```
