---
id: targets-egui-tab-order-taborder
kind: doc
target_file: src/targets/egui/tab_order.rs
line_hint: 20
author: 
created: 
title: "TabOrder"
tags: []
signature: "pub struct TabOrder"
---

Tab navigation order configuration for controls on an egui form.

What the form needs for its Tab order; built by [`plan`], `None` when no
control can take the focus.

# Examples

```rust
use crawrad_codegen::targets::egui::tab_order::plan;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("MainForm", "Form");
let to = plan(&reg, &doc, &[]);
assert!(to.is_none());
```
