---
id: targets-egui-components-status-bar-statuspanelcodegen
kind: doc
target_file: src/targets/egui/components/status_bar.rs
line_hint: 106
author: 
created: 
title: "StatusPanelCodegen"
tags: []
signature: "pub struct StatusPanelCodegen"
---

Code generation descriptor for StatusPanel.

`StatusPanel`, one panel of a StatusBar (#458): data the StatusBar draws,
so it has no body of its own.

# Examples

```rust
use crawrad_codegen::targets::egui::components::status_bar::StatusPanelCodegen;
use crawrad_codegen::targets::egui::components::EguiComponentCodegen;

let codegen = StatusPanelCodegen;
assert_eq!(codegen.class(), "StatusPanel");
```
